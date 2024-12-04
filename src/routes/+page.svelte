<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import {
    hideLoginWindow,
    logout,
    refreshInfo,
    showLoginWindow,
  } from "$lib/commands";
  import { onMount } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import type { Course } from "$lib/course";
  import { ChevronRight } from "lucide-svelte";
  import { userState } from "$lib/states.svelte";

  let dialogOpen = $state(false);

  let logined: boolean | null = $state(null); // null: not sure, true: logined, false: not logined

  function handleLoginClick() {
    dialogOpen = true;
  }

  async function handleOpenLoginWindow() {
    await showLoginWindow();
    dialogOpen = false;
  }

  async function handleLogoutClick() {
    await logout();
  }

  onMount(() => {
    let unlistenFoundCourses: UnlistenFn | null = null;
    listen<{ courses: Course[] }>("foundCourses", async (event) => {
      userState.courses = event.payload.courses.map((course) => ({
        ...course,
        selected: true, // default selected
      }));
      logined = true;
      // if user is not voting, hide login window
      if (!userState.voting) await hideLoginWindow();
    }).then((unlistenFn) => {
      unlistenFoundCourses = unlistenFn;
    });

    let unlistenNotLogin: UnlistenFn | null = null;
    listen("notLogin", (event) => {
      logined = false;
    }).then((unlistenFn) => {
      unlistenNotLogin = unlistenFn;
    });

    const refreshInterval = setInterval(() => {
      if (logined !== null) return;
      refreshInfo();
    }, 1000);

    return () => {
      if (unlistenFoundCourses) unlistenFoundCourses();
      if (unlistenNotLogin) unlistenNotLogin();
      clearInterval(refreshInterval);
    };
  });
</script>

{#if logined === null}
  <p class="text-sm opacity-80">Waiting...</p>
{:else if logined === false}
  <Button onclick={handleLoginClick}>Login with NTUST account</Button>
{:else if logined === true}
  {#if userState.courses !== undefined}
    <div class="flex flex-col gap-1 mt-10">
      <p>
        You have {userState.courses.length} courses that have not yet been evaluated.
      </p>
      <p>
        <Button variant="link" size="sm" onclick={handleLogoutClick}
          >Logout</Button
        >
        <Button
          variant="link"
          size="sm"
          onclick={() => {
            userState.voting = true;
            showLoginWindow();
          }}>Debug</Button
        >
      </p>
    </div>
    <div class="grow"></div>
    <div>
      <Button size="icon" href="select-courses"><ChevronRight /></Button>
    </div>
  {/if}
{/if}

<AlertDialog.Root bind:open={dialogOpen}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Please read these terms first</AlertDialog.Title>
      <AlertDialog.Description class="text-red-600">
        This application is not an official app of National Taiwan University of
        Science and Technology. Before logging into your account through a
        third-party application, please ensure you understand how the
        application works. This application will not steal your account. If you
        have any concerns, you are welcome to review and monitor the source code
        of this application.
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action on:click={handleOpenLoginWindow}
        >Continue</AlertDialog.Action
      >
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
