<script lang="ts">
	import { invalidateAll } from "$app/navigation";
	import type { PageData } from "./$types";

  export let data: PageData;
  let todos = data.todos;

  async function deleteTodo(id: number) {
    await fetch(`http://localhost:8181/delete/${id}`);
    invalidateAll();
  }

  async function updateTodo(todo: {
    id: number;
    description: string;
    done: boolean;
  }) {
    await fetch(`http://localhost:8181/update?\
id=${todo.id}&\
description=${todo.description}&\
done=${todo.done}`
    );
  }
</script>

<div class="container mx-auto mt-16">
  <h1 class="h1 text-center">Todos</h1>

  <form action="http://localhost:8181/create" method="POST">
    <input
      class="input p-4 my-8"
      name="description"
      type="text"
      placeholder="What needs to be done?"
    />
  </form>

  <div class="space-y-4">
    {#each data.todos as todo}
      <div
        class="flex items-center justify-between p-4 bg-surface-800 rounded-lg
gap-4"
      >
        <input
          class="checkbox"
          type="checkbox"
          bind:checked={todo.done}
          on:change={() => updateTodo(todo)}
        />

        <input
          class="input"  
          type="text"
          bind:value={todo.description}
          placeholder="Description"
        />

        <!-- <p>{todo.done}</p> -->

        <div class="flex gap-2">
          <button
            class="btn variant-filled-secondary"
            on:click={() => updateTodo(todo)}
          >
            Update
          </button>
          
          <button
            class="btn variant-filled-primary"
            on:click={() => deleteTodo(todo.id)}
          >
            Delete
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>