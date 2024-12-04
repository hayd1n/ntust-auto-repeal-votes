<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import * as Table from "$lib/components/ui/table/";
  import { onMount } from "svelte";
  import { resizeBig, resizeDefault } from "$lib/window";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { userState } from "$lib/states.svelte";

  onMount(() => {
    resizeBig();
    return () => {
      resizeDefault();
    };
  });
</script>

<p>Select courses</p>
<div class="overflow-y-auto overflow-x-hidden">
  <Table.Root class="text-left w-full">
    <Table.Header>
      <Table.Row>
        <Table.Head></Table.Head>
        <Table.Head>Course No</Table.Head>
        <Table.Head>Course Name</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#each userState.courses as { selected, courseName, courseNo }}
        <Table.Row>
          <Table.Cell><Checkbox bind:checked={selected} /></Table.Cell>
          <Table.Cell>{courseNo}</Table.Cell>
          <Table.Cell>{courseName}</Table.Cell>
        </Table.Row>
      {/each}
    </Table.Body>
  </Table.Root>
  <pre class="text-xs text-left">{JSON.stringify(
      userState.courses,
      null,
      2
    )}</pre>
</div>
<!-- <div class="grow"></div> -->
<div class="flex flex-row justify-center items-center gap-2">
  <Button variant="ghost" size="icon" onclick={() => history.back()}
    ><ChevronLeft /></Button
  >
  <Button size="icon" href="voting-confirm"><ChevronRight /></Button>
</div>
