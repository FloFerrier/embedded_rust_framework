# Connected Thermometer

## What It Is

The Connected Thermometer is a small, standalone sensor device that measures the temperature and relative humidity of its surroundings and shares them with your home automation setup. It runs on its own, needs no cables, no buttons, and no interaction once it is on: it continuously broadcasts the latest readings over the air.

It is designed to work with [Home Assistant](#home-assistant): after the device is powered on, Home Assistant detects it automatically and the temperature and humidity show up as regular sensors you can use in dashboards and automations — no pairing or configuration required.

## The System

Think of the device as a black box. On one side, it senses its environment; on the other, it broadcasts what it measured.

- **What it measures:** temperature and relative humidity.
- **What it does:** every few seconds it builds a small message with the current readings and broadcasts it over the radio.
- **What it does not do:** it does not connect to anything, does not answer requests, and keeps no history. It is fire-and-forget by design — the only thing the outside world ever sees is a periodic broadcast.

The device has no screen and no user interface. Its entire interface with the outside world is the radio broadcast described below.

## Interface With the Outside World

The broadcast uses Bluetooth Low Energy (BLE). The message format follows the [BTHome](https://bthome.io/) protocol, version 2.

Because the device speaks BTHome v2, any BTHome-compatible receiver can understand it. In practice, the consumer is Home Assistant: it picks up the broadcast automatically and exposes the temperature and humidity as sensors in its interface, ready to be displayed on dashboards or used in automations.

## Status

The current build is a proof of concept. This document describes the intended system and its behavior; the existing build does not yet implement it fully.

## Terminology

- **Bluetooth Low Energy (BLE):** the wireless radio standard the device uses to broadcast its readings.
- **Advertisement (broadcast):** a short, one-way radio message that announces data to any nearby listener. The device sends these periodically and no receiver needs to reply.
- **Advertising payload:** the data carried inside an advertisement.
- **BTHome:** a protocol that defines how sensor readings are packed into a broadcast so that home automation tools can decode them. The device uses version 2.
- **Home Assistant:** an open-source home automation platform that receives the device's broadcasts and turns them into usable sensors.
- **Sensor node:** a small, self-contained device that measures something — here, temperature and humidity — and shares the measurements over the air.