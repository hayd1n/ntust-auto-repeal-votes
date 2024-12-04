import { emit } from "@tauri-apps/api/event";
import { isLogined } from "./common";
import { getCourses } from "./courses";
import { isVoting } from "./voting";

let injected = false;

console.log("Test");

async function inject() {
  if (injected) return;

  console.log("Injecting...");

  const voting = isVoting();
  await emit("votingState", voting);

  if (!isLogined()) {
    console.log("Not logined!");
    await emit("notLogin");
    return;
  }

  if (voting) return;

  const courses = await getCourses();
  console.log(courses);

  document.body.appendChild(document.createElement("div")).innerText =
    JSON.stringify(courses);

  await emit("foundCourses", { courses });

  console.log("Injected!");
  injected = true;
}

document.addEventListener("DOMContentLoaded", inject);
document.addEventListener("load", inject);
inject();
