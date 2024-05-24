<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { WebviewWindow, currentMonitor } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { sleep } from "../common";
  import Button from "$lib/components/ui/button/button.svelte";
  import { Separator } from "$lib/components/ui/separator";
  import { appWindow } from "@tauri-apps/api/window";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";

  const HOME_URL =
    "https://stuinfosys.ntust.edu.tw/JudgeCourseServ/JudgeCourse/ListJudge";
  const POST_URL =
    "https://stuinfosys.ntust.edu.tw/JudgeCourseServ/JudgeQuestion/ListJudge";

  type CourseInfo = {
    courseNo: string;
    courseName: string;
    payload: string;
  };

  let innerHeight = 0;

  let url = "unknown";

  let courses: CourseInfo[];

  let logined = false;

  let topBarRef: HTMLElement | null = null;

  let readyDialogShow = false;

  let dryRun = false;

  $: scrollAreaHeight = topBarRef ? innerHeight - topBarRef.clientHeight : 0;

  async function findCourses() {
    await invoke("find_courses");
  }

  async function handleStartVotes() {
    await invoke("start_votes", {
      homeUrl: HOME_URL,
      postUrl: POST_URL,
      courses: courses,
    });
  }

  async function handleReadyToVotes() {
    await appWindow.center();
    await appWindow.setAlwaysOnTop(true);
    await appWindow.setFocus();
    readyDialogShow = true;
  }

  async function handleCancelVotes() {
    await invoke("close_courses", {
      courses: courses,
    });

    await appWindow.setAlwaysOnTop(false);
  }

  async function handleSubmitVotes() {
    await invoke("submit_courses", {
      homeUrl: HOME_URL,
      courses: courses,
      dryRun,
    });
  }

  async function handleSubmitComplete() {
    await sleep(1000);

    await invoke("close_courses", {
      courses: courses,
    });

    await sleep(1000);

    await invoke("redirect_ntust", {
      url: HOME_URL,
    });

    await appWindow.setAlwaysOnTop(false);

    const ntustWindow = WebviewWindow.getByLabel("ntust");
    ntustWindow?.setFocus();
  }

  onMount(() => {
    let interval = setInterval(async () => {
      url = await invoke("get_url");

      let currentLogined = url.includes(HOME_URL);
      if (currentLogined && !logined) {
        await sleep(1000);
        findCourses();
      }
      logined = currentLogined;

      if (url.includes("#courses")) {
        let part = url.split("#courses")[1];
        let decoded = decodeURI(part);
        courses = JSON.parse(decoded);
      } else {
        courses = [];
      }
    }, 1000);

    let unlistenReadyToSubmit: UnlistenFn | null = null;
    listen("readyToSubmit", handleReadyToVotes).then(
      (u) => (unlistenReadyToSubmit = u)
    );

    let unlistenSubmitComplete: UnlistenFn | null = null;
    listen("submitComplete", handleSubmitComplete).then(
      (u) => (unlistenSubmitComplete = u)
    );

    return () => {
      clearInterval(interval);
      if (unlistenReadyToSubmit) unlistenReadyToSubmit();
      if (unlistenSubmitComplete) unlistenSubmitComplete();
    };
  });
</script>

<svelte:window bind:innerHeight />

{#if logined}
  <div class="w-screen h-screen flex flex-col justify-start">
    <div class="w-full p-4 text-center" bind:this={topBarRef}>
      <Button class="w-full" on:click={() => handleStartVotes()}
        >Start Votes 開始投票</Button
      >
      <!-- svelte-ignore a11y-invalid-attribute -->
      <a
        href="javascript:;"
        on:click={() => {
          dryRun = true;
          handleStartVotes();
        }}>Dry Run</a
      >
    </div>
    <div>
      <!-- <pre style="text-align: left;">{JSON.stringify(courses, null, 2)}</pre> -->
      <div
        class="w-full py-4 px-8 overflow-y-auto"
        style="height: {scrollAreaHeight}px;"
      >
        <h4 class="mb-4 text-sm font-medium leading-none">Courses</h4>
        {#each courses as course, i}
          <div class="flex flex-nowrap justify-between items-center text-sm">
            <div>{course.courseNo}</div>
            <div>{course.courseName}</div>
          </div>
          <Separator class="my-2" />
        {/each}
      </div>
    </div>
  </div>

  <AlertDialog.Root bind:open={readyDialogShow}>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>You are ready to go!</AlertDialog.Title>
        <AlertDialog.Description>
          This action cannot be undone.
          {#if dryRun}
            <p class="text-red-500">This is dry run.</p>
          {/if}
        </AlertDialog.Description>
      </AlertDialog.Header>
      <AlertDialog.Footer>
        <AlertDialog.Cancel on:click={handleCancelVotes}
          >Cancel</AlertDialog.Cancel
        >
        <AlertDialog.Action on:click={handleSubmitVotes}
          >Let's go</AlertDialog.Action
        >
      </AlertDialog.Footer>
    </AlertDialog.Content>
  </AlertDialog.Root>
{:else}
  <div class="w-screen h-screen flex flex-col justify-center items-center">
    <div class="flex">
      <div class="text-center">
        <h1>Waiting for login...</h1>
        <h1>等待手動登入中...</h1>
      </div>
    </div>
  </div>
{/if}
<!-- <p>{monitorSize.width}, {monitorSize.height}, {monitorSize.scaleFactor}</p> -->
<!-- <p>{url}</p> -->
<!-- <Button on:click={() => invoke("do_vote")}>Do Vote</Button> -->
<!-- <Button on:click={handleFindCourses}>Find Courses</Button> -->
<!-- <Button on:click={handleStartVotes}>Start Votes</Button> -->
