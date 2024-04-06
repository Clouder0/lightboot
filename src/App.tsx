import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import {
  useQuery,
} from '@tanstack/react-query'
import "./globals.css";

function BottomBar(props: { max_idx: number; now_idx: number }) {
  return (
    <div className="h-6 bg-slate-50 flex-none">
      {props.now_idx}, {props.max_idx}
    </div>
  );
}

function ItemEntry(props: { name: string; start_cmd: string}) {
  return <div className="bg-slate-300 border-2" onClick={() => {
    console.log("execute command", props.start_cmd);
  }}>{props.name}</div>;
}

function ItemGrid(props: { items?: any }) {
  {
    return (
      <div
        className="grid grid-cols-4 grid-rows-4 w-auto h-auto flex-grow"
      >
        {props.items?.map((item: any) => (<ItemEntry name={item.name} start_cmd={item.start_cmd}></ItemEntry>))}
      </div>
    );
  }
}

function Menu(props: {focus_func: () => void, now_idx: number, max_idx: number, items?: any}) {
  return (
    <div
      className="bg-slate-700 flex-1 flex flex-col"
      onMouseEnter={props.focus_func}
    >
      <ItemGrid items={props.items}></ItemGrid>
      <BottomBar max_idx={props.max_idx} now_idx={props.now_idx} />
    </div>
  );
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  
  const maxupidx = 10, maxdownidx = 10;
  const [upidx,setUpidx] = useState(0); 
  const [downidx,setDownidx] = useState(0); 
  listen("middle_click", (event) => {
    console.log(event);
    setGreetMsg("Middle click detected");
  });

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  const [focusMenu, setFocusMenu] = useState(0);
  useEffect(() => {
    console.log("add");
    const onwheel = (e: Event) => {
      const direction = (e as unknown as {deltaY: number}).deltaY > 0 ? 1 : -1; // 1 for down, -1 for up
      if(focusMenu === 0) {
        if(upidx + direction >= 0 && upidx + direction <= maxupidx) {
          setUpidx(upidx + direction);
        }
      } else {
        if(downidx + direction >= 0 && downidx + direction <= maxdownidx) {
          setDownidx(downidx + direction);
        }
      }
      console.log(`scroll with menu ${focusMenu}`, e);
    }
    window.addEventListener("mousewheel", onwheel);    
    return () => {
      window.removeEventListener("mousewheel", onwheel);
    }
  }, [focusMenu, upidx, downidx]);
  
  const up_query = useQuery({ queryKey: ['up', upidx], queryFn: async() => {
    return await invoke("upmenuquery", {idx: upidx});
  }});
  
  const down_query = useQuery({ queryKey: ['down', downidx], queryFn: async() => {
    return await invoke("downmenuquery", {idx: downidx});
  }});

  return (
    <div
      className="bg-slate-400 w-screen h-screen flex flex-col"
    >
      <Menu items={up_query.data} now_idx={upidx} max_idx={maxupidx} focus_func={() => {setFocusMenu(0)}}></Menu>
      <Menu items={down_query.data} now_idx={downidx} max_idx={maxdownidx} focus_func={() => {setFocusMenu(1)}}/>
    </div>
  );
}

export default App;
