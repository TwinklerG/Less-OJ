"use client"

import * as React from "react"
import {
  AudioWaveform,
  BookOpen,
  Bot,
  Command,
  Frame,
  GalleryVerticalEnd,
  Map,
  PieChart,
  Settings2,
  SquareTerminal,
} from "lucide-react"

import { NavMain } from "@/components/nav-main"
import { NavProjects } from "@/components/nav-projects"
import { NavUser } from "@/components/nav-user"
import { TeamSwitcher } from "@/components/team-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar"

// This is sample data.
const data = {
  user: {
    name: "shadcn",
    email: "m@example.com",
    avatar: "/avatars/shadcn.jpg",
  },
  courses: [
    {
      name: "C语言程序设计",
      logo: GalleryVerticalEnd,
      plan: "CPL",
    },
    {
      name: "软件工程与计算1",
      logo: AudioWaveform,
      plan: "SE1",
    },
    {
      name: "数据结构与算法",
      logo: Command,
      plan: "DSA",
    },
  ],
  assignments: [
    {
      title: "作业一",
      url: "#",
      icon: SquareTerminal,
      isActive: true,
      items: [
        {
          title: "题目一",
          url: "#",
        },
        {
          title: "题目二",
          url: "#",
        },
        {
          title: "题目三",
          url: "#",
        },
      ],
    },
    {
      title: "作业二",
      url: "#",
      icon: Bot,
      items: [
        {
          title: "题目一",
          url: "#",
        },
        {
          title: "题目二",
          url: "#",
        },
        {
          title: "题目三",
          url: "#",
        },
      ],
    },
    {
      title: "作业三",
      url: "#",
      icon: BookOpen,
      items: [
        {
          title: "题目一",
          url: "#",
        },
        {
          title: "题目二",
          url: "#",
        },
        {
          title: "题目三",
          url: "#",
        },
      ],
    },
    {
      title: "作业四",
      url: "#",
      icon: Settings2,
      items: [
        {
          title: "题目一",
          url: "#",
        },
        {
          title: "题目二",
          url: "#",
        },
        {
          title: "题目三",
          url: "#",
        },
      ],
    },
  ],
  exams: [
    {
      name: "考试一",
      url: "#",
      icon: Frame,
    },
    {
      name: "考试二",
      url: "#",
      icon: PieChart,
    },
    {
      name: "考试三",
      url: "#",
      icon: Map,
    },
  ],
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <TeamSwitcher teams={data.courses} />
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={data.assignments} />
        <NavProjects projects={data.exams} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={data.user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
