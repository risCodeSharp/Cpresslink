<template>
    <div class="bg-white p-6 shadow-[0px_0px_48px_-2px_rgba(0,0,0,0.1)] rounded-xl  transition duration-450 ease-in-out hover:-translate-y-1">
        <div class="p-2 bg-[#e8f0ea] w-9 h-9 flex justify-center items-center rounded-lg">
            <span class="text-[#2d5a3d]">
                <slot name="icon" />
            </span>
        </div>
        <div class="mt-3 font-semibold tracking-wide text-[#1a2e23]">
            <slot name="header"/>
        </div>
        <div class="my-2 text-mist-500 text-sm">
            <slot name="body"/>
        </div>
    </div>
</template>
<style lang="css" scoped>
    
</style>