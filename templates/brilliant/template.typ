// =============================================================================
// Brilliant Resume Template — CodeResume
// Advanced design with customizable colors, skill indicators, and visual flair
// Perfect for tech professionals seeking a sophisticated, modern look
// =============================================================================

// --- Configuration -----------------------------------------------------------
#let lang = sys.inputs.at("lang", default: "en")
#let data-path = sys.inputs.at("data-path", default: "resume-data.json")
#let data = json(data-path)

// --- Language labels ---------------------------------------------------------
#let L = if lang == "zh" {
  (
    experience: "工作经历",
    education: "教育背景",
    skills: "专业技能",
    projects: "项目经历",
    certifications: "证书资质",
    publications: "发表论文",
    opensource: "开源贡献",
    present: "至今",
    summary: "个人简介",
  )
} else {
  (
    experience: "Experience",
    education: "Education",
    skills: "Technical Skills",
    projects: "Projects",
    certifications: "Certifications",
    publications: "Publications",
    opensource: "Open Source",
    present: "Present",
    summary: "Summary",
  )
}

// --- Advanced Theme System (Customizable color themes) ----------------------
#let theme = (
  // Primary accent color — change to customize (e.g., #e74c3c for red)
  accent: rgb("#2563eb"),
  // Lighter variant for backgrounds
  accent-light: rgb("#dbeafe"),
  // Secondary accent for variety
  accent-dark: rgb("#1e40af"),
  // Neutral colors for hierarchy
  text-primary: rgb("#1f2937"),
  text-secondary: rgb("#6b7280"),
  text-muted: rgb("#9ca3af"),
  // Backgrounds and accents
  bg-light: rgb("#f9fafb"),
  bg-accent: rgb("#eff6ff"),
  // Skill rating colors (good/medium/learning progression)
  skill-expert: rgb("#059669"),     // Green for expert
  skill-advanced: rgb("#0891b2"),   // Cyan for advanced
  skill-intermediate: rgb("#2563eb"), // Blue for intermediate
  skill-learning: rgb("#f59e0b"),   // Amber for learning
)

// --- Font selection ----------------------------------------------------------
#let body-font = if lang == "zh" {
  ("PingFang SC", "Hiragino Sans GB", "Heiti SC", "Noto Sans CJK SC", "Source Han Sans SC")
} else {
  ("Inter", "Helvetica Neue", "Arial", "Liberation Sans")
}

// --- Page setup -------------------------------------------------------------- 
#set page(
  paper: "a4",
  margin: (top: 1.1cm, bottom: 1.1cm, left: 1.3cm, right: 1.3cm),
)
#set text(font: body-font, size: 9.5pt, fill: theme.text-primary)
#set par(leading: 0.58em, justify: false)

// --- Helper functions --------------------------------------------------------

#let fmt-date(d) = {
  if d == none { return L.present }
  if type(d) == str { return d }
  if type(d) == int { return str(d) }
  if type(d) == dictionary {
    if "month" in d and "year" in d {
      let months-en = ("Jan", "Feb", "Mar", "Apr", "May", "Jun",
                        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec")
      if lang == "zh" {
        return str(d.year) + "年" + str(d.month) + "月"
      } else {
        return months-en.at(d.month - 1) + " " + str(d.year)
      }
    }
    if "year" in d { return str(d.year) }
  }
  return str(d)
}

#let date-range(start, end) = {
  text(fill: theme.text-secondary, size: 8.5pt)[#fmt-date(start) — #fmt-date(end)]
}

// Skill rating bar — visual proficiency indicator
#let skill-bar(name, level: 3) = {
  // level: 1=learning, 2=intermediate, 3=advanced, 4=expert
  let colors = (theme.skill-learning, theme.skill-intermediate, theme.skill-advanced, theme.skill-expert)
  let color = colors.at(int(level) - 1)
  let filled = if level == 1 { "●" } else if level == 2 { "●●" } else if level == 3 { "●●●" } else { "●●●●" }
  let unfilled = if level == 4 { "" } else if level == 3 { "○" } else if level == 2 { "○○" } else { "○○○" }
  
  box(
    fill: theme.bg-light,
    radius: 3pt,
    inset: (x: 6pt, y: 3pt),
    [
      #text(size: 8.8pt, weight: "regular")[#name]
      #h(6pt)
      #text(size: 7.5pt, fill: color, weight: "bold")[#filled]#text(size: 7.5pt, fill: theme.text-muted)[#unfilled]
    ]
  )
}

// Accent pill for technologies
#let pill(content) = {
  box(
    fill: theme.bg-accent,
    radius: 4pt,
    inset: (x: 5pt, y: 2.5pt),
    text(size: 7.9pt, fill: theme.accent, weight: "500")[#content]
  )
}

// Section heading with accent bar
#let section-heading(title) = {
  v(0.55em)
  box(
    width: 100%,
    inset: (bottom: 0.2em),
    [
      #text(size: 12.5pt, weight: "bold", fill: theme.accent)[#title]
      #line(length: 100%, stroke: 1.2pt + theme.accent)
    ]
  )
  v(0.3em)
}

// =============================================================================
// HEADER — Premium Design
// =============================================================================
#align(center)[
  #{
    let photo = data.personal.at("photo", default: none)
    if photo != none {
      box(
        stroke: 2pt + theme.accent,
        radius: 50%,
        clip: true,
        image(photo, width: 2.5cm, height: 2.5cm, fit: "cover")
      )
      v(0.3em)
    }
  }
  #text(size: 24pt, weight: "bold", fill: theme.accent)[#data.personal.name]
  #{
    let na = data.personal.at("name-alt", default: none)
    if na != none {
      text(size: 14pt, fill: theme.text-secondary, " / " + na)
    }
  }
  #v(0.2em)
  #if "title" in data.personal and data.personal.title != none {
    text(size: 11.5pt, fill: theme.text-secondary, weight: "500")[#data.personal.title]
    v(0.25em)
  }
  #text(size: 8.5pt, fill: theme.text-muted)[
    #data.personal.email
    #if "phone" in data.personal and data.personal.phone != none [ · #data.personal.phone]
    #if "location" in data.personal and data.personal.location != none [ · #data.personal.location]
    #if "github" in data.personal and data.personal.github != none [ · #link(data.personal.github)[GitHub]]
    #if "linkedin" in data.personal and data.personal.linkedin != none [ · #link(data.personal.linkedin)[LinkedIn]]
    #if "website" in data.personal and data.personal.website != none [ · #link(data.personal.website)[Website]]
  ]
]

// =============================================================================
// SUMMARY
// =============================================================================
#if "summary" in data and data.summary != none {
  v(0.3em)
  box(
    fill: theme.bg-light,
    radius: 4pt,
    inset: 8pt,
    text(size: 9.2pt, fill: rgb("#374151"), style: "italic")[#data.summary]
  )
  v(0.2em)
}

// =============================================================================
// EXPERIENCE
// =============================================================================
#if "experience" in data and data.experience.len() > 0 {
  section-heading(L.experience)
  for exp in data.experience {
    grid(
      columns: (1fr, auto),
      [
        #text(weight: "bold", size: 10.2pt, fill: theme.accent)[#exp.title]
        #text(fill: theme.text-secondary, size: 9.3pt)[ at #exp.company]
        #if "location" in exp and exp.location != none {
          text(fill: theme.text-muted, size: 8.5pt)[ · #exp.location]
        }
      ],
      align(right, date-range(exp.at("start-date"), exp.at("end-date", default: none)))
    )
    v(0.12em)
    for bullet in exp.at("highlights", default: ()) {
      [- #text(size: 9.1pt)[#bullet]]
    }
    if exp.at("technologies", default: ()).len() > 0 {
      v(0.1em)
      for tech in exp.technologies { pill(tech); h(3pt) }
    }
    v(0.25em)
  }
}

// =============================================================================
// EDUCATION
// =============================================================================
#if "education" in data and data.education.len() > 0 {
  section-heading(L.education)
  for edu in data.education {
    grid(
      columns: (1fr, auto),
      [
        #text(weight: "bold", size: 10.2pt, fill: theme.accent)[#edu.institution]
        #text(fill: theme.text-secondary, size: 9.3pt)[ · #edu.degree]
        #if "gpa" in edu and edu.gpa != none {
          text(fill: theme.text-muted, size: 8.5pt)[ · GPA: #edu.gpa]
        }
      ],
      align(right, date-range(edu.at("start-date"), edu.at("end-date", default: none)))
    )
    for bullet in edu.at("highlights", default: ()) {
      [- #text(size: 9.1pt)[#bullet]]
    }
    v(0.2em)
  }
}

// =============================================================================
// SKILLS — Advanced with rating indicators
// =============================================================================
#if "skills" in data and data.skills.at("groups", default: ()).len() > 0 {
  section-heading(L.skills)
  for group in data.skills.groups {
    text(weight: "bold", size: 9.5pt, fill: theme.text-primary)[#group.category]
    v(0.1em)
    for (i, skill) in group.skills.enumerate() {
      skill-bar(skill, level: 3)
      if (i + 1) < group.skills.len() { h(3pt) }
    }
    v(0.2em)
  }
}

// =============================================================================
// PROJECTS
// =============================================================================
#if "projects" in data and data.projects.len() > 0 {
  section-heading(L.projects)
  for proj in data.projects {
    [
      #text(weight: "bold", size: 10.2pt, fill: theme.accent)[#proj.name]
      #if "url" in proj and proj.url != none {
        text(fill: theme.text-secondary, size: 8pt)[ · #link(proj.url)[↗]]
      }
      #if "role" in proj and proj.role != none {
        text(fill: theme.text-secondary, size: 9.3pt)[ · #proj.role]
      }
    ]
    v(0.1em)
    text(size: 9.2pt)[#proj.description]
    for bullet in proj.at("highlights", default: ()) {
      [- #text(size: 9.1pt)[#bullet]]
    }
    if proj.at("technologies", default: ()).len() > 0 {
      v(0.1em)
      for tech in proj.technologies { pill(tech); h(3pt) }
    }
    v(0.25em)
  }
}

// =============================================================================
// OPEN SOURCE
// =============================================================================
#if "open-source" in data and data.at("open-source").len() > 0 {
  section-heading(L.opensource)
  for oss in data.at("open-source") {
    [
      #text(weight: "bold", size: 10.2pt, fill: theme.accent)[#oss.project]
      #text(fill: theme.text-secondary, size: 9.3pt)[ · #oss.role]
    ]
    v(0.1em)
    text(size: 9.2pt)[#oss.description]
    v(0.2em)
  }
}

// =============================================================================
// CERTIFICATIONS
// =============================================================================
#if "certifications" in data and data.certifications.len() > 0 {
  section-heading(L.certifications)
  for cert in data.certifications {
    box(
      fill: theme.bg-accent,
      radius: 3pt,
      inset: (x: 5pt, y: 2pt),
      [
        #text(weight: "bold", size: 9.3pt, fill: theme.accent)[#cert.name]
        #text(fill: theme.text-secondary, size: 8.8pt)[ — #cert.issuer]
      ]
    )
    v(0.12em)
  }
}

// =============================================================================
// PUBLICATIONS
// =============================================================================
#if "publications" in data and data.publications.len() > 0 {
  section-heading(L.publications)
  for p in data.publications {
    [#text(weight: "bold", size: 10.2pt, fill: theme.accent)[#p.title]]
    v(0.08em)
    text(size: 9pt, fill: theme.text-secondary)[
      #if "authors" in p and p.authors != none [#p.authors · ]
      _#p.venue_
      #if "date" in p and p.date != none [, #fmt-date(p.date)]
      #if "url" in p and p.url != none [ · #link(p.url)[[link]]]
    ]
    v(0.18em)
  }
}
