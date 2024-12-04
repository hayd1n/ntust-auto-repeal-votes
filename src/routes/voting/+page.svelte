<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { userState } from "$lib/states.svelte";
  import { voting } from "$lib/commands";

  onMount(async () => {
    const selectedCourses = userState.courses.filter((c) => c.selected);
    await voting(selectedCourses);
  });
</script>

<div class="flex flex-col gap-1 mt-10">
  <p>Voting...</p>
</div>
<div class="grow"></div>
<div class="flex flex-row justify-center items-center gap-2">
  <Button variant="ghost" size="icon" onclick={() => history.back()}
    ><ChevronLeft /></Button
  >
</div>
