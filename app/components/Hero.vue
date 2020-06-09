<template>
    <header class="hero">
        <svg class="hero__background" viewBox="0 0 1024 600" preserveAspectRatio="xMinYMid slice" version="1.1"
             xmlns="http://www.w3.org/2000/svg">
            <defs>
                <radialGradient cx="-3.53597005%" cy="-3.28903198%" fx="-3.53597005%" fy="-3.28903198%" r="100.316897%"
                                gradientTransform="translate(-0.035360,-0.032890),scale(0.999024,1.000000),rotate(26.625265),translate(0.035360,0.032890)"
                                id="radialGradient-1">
                    <stop stop-color="#F67280" offset="0%"></stop>
                    <stop stop-color="#355C7D" stop-opacity="0" offset="100%"></stop>
                </radialGradient>
                <radialGradient cx="94.7254181%" cy="50%" fx="94.7254181%" fy="50%" r="71.1748657%"
                                gradientTransform="translate(0.947254,0.500000),scale(0.999024,1.000000),rotate(-150.756225),translate(-0.947254,-0.500000)"
                                id="radialGradient-2">
                    <stop stop-color="#F8B195" offset="0%"></stop>
                    <stop stop-color="#355C7D" stop-opacity="0" offset="100%"></stop>
                </radialGradient>
                <filter x="0.0%" y="0.0%" width="100.0%" height="100.0%" filterUnits="objectBoundingBox" id="filter-1">
                    <feGaussianBlur stdDeviation="0" in="SourceGraphic"></feGaussianBlur>
                </filter>
            </defs>
            <polygon points="0 0 1024 0 1024 600 0 600" fill="#355C7D"></polygon>
            <rect fill="url(#radialGradient-1)" opacity="0.8" x="0" y="0" width="1024" height="1024"></rect>
            <rect fill="url(#radialGradient-2)" opacity="0.6" filter="url(#filter-1)" x="0" y="0" width="1024"
                  height="1024"></rect>
        </svg>
        <div class="hero-label">
            <svg class="hero-label__border" viewBox="0 0 600 400" preserveAspectRatio="none" version="1.1"
                 xmlns="http://www.w3.org/2000/svg">
                <defs>
                    <linearGradient x1="63.7109375%" y1="26.6983111%" x2="45.2475586%" y2="73.3016889%"
                                    id="linearGradient">
                        <stop stop-color="#6C5B7B" offset="0%"></stop>
                        <stop stop-color="#C06C84" offset="100%"></stop>
                    </linearGradient>
                </defs>
                <rect x="0" y="0" width="600" height="400" fill-opacity="0.07" fill="#FFFFFF"
                      stroke="url(#linearGradient)" stroke-width="24" vector-effect="non-scaling-stroke"></rect>
            </svg>
            <h1 class="hero__title has-font-timberline">
                Lorem ipsum dolor sit
            </h1>
            <p class="is-center h4 hero-label__text">
                Suspendisse ac diam libero. Morbi viverra orci et arcu convallis imperdiet. Etiam vehicula nisi dolor.
            </p>
            <div class="is-center">
                <a href="/" class="button">
                    Nam pretium vitae urna id
                </a>
            </div>
        </div>
    </header>
</template>

<script lang="ts">
    export default {
        name: "Hero"
    }
</script>

<style scoped>
    .hero {
        display: block;
        width: 100vw;
        position: relative;
        z-index: 1;
        padding-top: 120px;
        background-color: hsl(208, 40%, 35%);
        min-height: 70vh;
        padding-bottom: var(--section-margin-height);
    }

    .hero__background {
        position: absolute;
        display: block;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
    }

    .hero__title {
        position: absolute;
        top: 42px;
        left: 50%;
        font-size: calc(50px + (110 - 50) * (100vw - 320px) / (1440 - 320)) !important;
        line-height: 1;
        color: #F8B195;
        -moz-osx-font-smoothing: grayscale;
        letter-spacing: 0;
        font-weight: 500;
        margin: 0;
        white-space: nowrap;
        transform: translateX(-50%);
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.16);
    }

    .hero-label {
        position: relative;
        display: block;
        padding: calc(50px + (110 - 50) * (100vw - 320px) / (1440 - 320) + 42px + 35px) 42px 42px;
        max-width: 600px;
        margin: var(--layout-gutter-width) auto;
    }

    .hero-label__text {
        position: relative;
        letter-spacing: -0.56px;
    }

    .hero-label .button {
        margin-top: 40px;
    }

    @media screen and (max-width: 550px) {
        .hero__title {
            display: none;
        }

        .hero-label {
            padding-top: 42px;
        }
    }

    .hero-label__border {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
    }

</style>