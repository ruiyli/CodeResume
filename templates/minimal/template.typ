// Minimal Resume Template — CodeResume (Enhanced)
// Ultra-clean, maximum whitespace with improved visual hierarchy

#let lang = sys.inputs.at("lang", default: "en")
#let data-path = sys.inputs.at("data-path", default: "resume-data.json")
#let data = json(data-path)

#let L = if lang == "zh" {
  (experience: "工作经历", education: "教育背景", skills: "技能",
   projects: "项目", present: "至今", summary: "简介")
} else {
  (experience: "Experience", education: "Education", skills: "Skills",
   projects: "Projects", present: "Present", summary: "Summary")
}

#let muted = rgb("#888888")
#let accent = rgb("#333333")

#let body-font = if lang == "zh" {
  ("PingFang SC", "Hiragino Sans GB", "Heiti SC", "Noto Sans CJK SC")
} else {
  ("Helvetica Neue", "Inter", "Arial", "Liberation Sans")
}

#set page(paper: "a4", margin: (top: 1.4cm, bottom: 1.4cm, left: 1.8cm, right: 1.8cm))
#set text(font: body-font, size: 9pt, fill: rgb("#222222"))
#set par(leading: 0.55em)

#let fmt-date(d) = {
  if d == none { return L.present }
  if type(d) == str { return d }
  if type(d) == int { return str(d) }
  if type(d) == dictionary {
    if "month" in d and "year" in d {
      let months = ("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec")
      if lang == "zh" { return str(d.year) + "." + str(d.month) }
      else { return months.at(d.month - 1) + " " + str(d.year) }
    }
    if "year" in d { return str(d.year) }
  }
  return str(d)
}

#let section-heading(title) = {
  v(0.8em)
  // Enhanced: bold instead of smallcaps, larger size
  text(size: 10.5pt, weight: "bold", fill: accent)[#title]
  v(0.35em)
}

// HEADER
#{
  let photo = data.personal.at("photo", default: none)
  if photo != none {
    grid(columns: (auto, 1fr), column-gutter: 0.8cm, align: (center + horizon, left + horizon),
      box(radius: 50%, clip: true,
        image(photo, width: 2cm, height: 2cm, fit: "cover")),
      [
        #text(size: 17pt, weight: "bold", tracking: 0.02em)[#data.personal.name]
        #v(0.15em)
        #text(size: 8.3pt, fill: muted)[
          #data.personal.email
          #if "phone" in data.personal and data.personal.phone != none [ · #data.personal.phone]
          #if "location" in data.personal and data.personal.location != none [ · #data.personal.location]
          #if "github" in data.personal and data.personal.github != none [ · #link(data.personal.github)[GitHub]]
          #if "linkedin" in data.personal and data.personal.linkedin != none [ · #link(data.personal.linkedin)[LinkedIn]]
        ]
      ]
    )
  } else {
    text(size: 17pt, weight: "bold", tracking: 0.02em)[#data.personal.name]
    v(0.1em)
    text(size: 8.3pt, fill: muted)[
      #data.personal.email
      #if "phone" in data.personal and data.personal.phone != none [ · #data.personal.phone]
      #if "location" in data.personal and data.personal.location != none [ · #data.personal.location]
      #if "github" in data.personal and data.personal.github != none [ · #link(data.personal.github)[GitHub]]
      #if "linkedin" in data.personal and data.personal.linkedin != none [ · #link(data.personal.linkedin)[LinkedIn]]
    ]
  }
}

// SUMMARY
#if "summary" in data and data.summary != none {
  v(0.4em)
  text(size: 8.8pt, fill: rgb("#555555"))[#data.summary]
}

// EXPERIENCE
#if "experience" in data and data.experience.len() > 0 {
  section-heading(L.experience)
  for exp in data.experience {
    grid(columns: (1fr, auto),
      [#text(weight: "bold", size: 9.3pt)[#exp.title] #text(fill: muted, size: 8.9pt)[at #exp.company]],
      align(right, text(size: 8pt, fill: muted)[#fmt-date(exp.at("start-date")) — #fmt-date(exp.at("end-date", default: none))]))
    v(0.08em)
    for bullet in exp.at("highlights", default: ()) { [- #text(size: 8.5pt)[#bullet]] }
    v(0.3em)
  }
}

// EDUCATION
#if "education" in data and data.education.len() > 0 {
  section-heading(L.education)
  for edu in data.education {
    grid(columns: (1fr, auto),
      [#text(weight: "bold", size: 9.3pt)[#edu.institution] #text(fill: muted, size: 8.9pt)[#edu.degree]],
      align(right, text(size: 8pt, fill: muted)[#fmt-date(edu.at("start-date")) — #fmt-date(edu.at("end-date", default: none))]))
    v(0.15em)
  }
}

// SKILLS
#if "skills" in data and data.skills.at("groups", default: ()).len() > 0 {
  section-heading(L.skills)
  for group in data.skills.groups {
    text(size: 8.5pt)[*#group.category* #group.skills.join(" · ")]
    v(0.08em)
  }
}

// PROJECTS
#if "projects" in data and data.projects.len() > 0 {
  section-heading(L.projects)
  for proj in data.projects {
    text(weight: "bold", size: 9.3pt)[#proj.name]
    v(0.03em)
    text(size: 8.5pt)[#proj.description]
    v(0.25em)
  }
}
