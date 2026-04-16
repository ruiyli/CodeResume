// Classic Resume Template — CodeResume
// Traditional, conservative layout

#let lang = sys.inputs.at("lang", default: "en")
#let data-path = sys.inputs.at("data-path", default: "resume-data.json")
#let data = json(data-path)

#let L = if lang == "zh" {
  (experience: "工作经历", education: "教育背景", skills: "专业技能",
   projects: "项目经历", certifications: "证书资质", opensource: "开源贡献",
   present: "至今", summary: "个人简介")
} else {
  (experience: "Experience", education: "Education", skills: "Skills",
   projects: "Projects", certifications: "Certifications", opensource: "Open Source",
   present: "Present", summary: "Summary")
}

#let muted = rgb("#555555")

#let body-font = if lang == "zh" {
  ("Songti SC", "PingFang SC", "Heiti SC", "Noto Serif CJK SC")
} else {
  ("Georgia", "Times New Roman", "Liberation Serif")
}

#set page(paper: "a4", margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm))
#set text(font: body-font, size: 10pt, fill: rgb("#333333"))
#set par(leading: 0.65em)

#let fmt-date(d) = {
  if d == none { return L.present }
  if type(d) == str { return d }
  if type(d) == int { return str(d) }
  if type(d) == dictionary {
    if "month" in d and "year" in d {
      let months = ("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec")
      if lang == "zh" { return str(d.year) + "年" + str(d.month) + "月" }
      else { return months.at(d.month - 1) + " " + str(d.year) }
    }
    if "year" in d { return str(d.year) }
  }
  return str(d)
}

#let section-heading(title) = {
  v(0.8em)
  text(size: 12pt, weight: "bold")[#upper(title)]
  v(0.2em)
  line(length: 100%, stroke: 1pt + rgb("#333333"))
  v(0.4em)
}

// HEADER
#align(center)[
  #{
    let photo = data.personal.at("photo", default: none)
    if photo != none {
      box(radius: 50%, clip: true,
        image(photo, width: 2.4cm, height: 2.4cm, fit: "cover"))
      v(0.3em)
    }
  }
  #text(size: 20pt, weight: "bold")[#data.personal.name]
  #if "name-alt" in data.personal and data.personal.at("name-alt") != none {
    text(size: 12pt, fill: muted)[ (#data.personal.at("name-alt"))]
  }
  #v(0.3em)
  #if "title" in data.personal and data.personal.title != none {
    text(size: 11pt, fill: muted)[#data.personal.title]
    v(0.2em)
  }
  #text(size: 9pt, fill: muted)[
    #data.personal.email
    #if "phone" in data.personal and data.personal.phone != none [ | #data.personal.phone]
    #if "location" in data.personal and data.personal.location != none [ | #data.personal.location]
    #if "github" in data.personal and data.personal.github != none [ | #link(data.personal.github)[GitHub]]
    #if "linkedin" in data.personal and data.personal.linkedin != none [ | #link(data.personal.linkedin)[LinkedIn]]
  ]
]

// SUMMARY
#if "summary" in data and data.summary != none {
  section-heading(L.summary)
  text(size: 9.5pt)[#data.summary]
}

// EXPERIENCE
#if "experience" in data and data.experience.len() > 0 {
  section-heading(L.experience)
  for exp in data.experience {
    grid(columns: (1fr, auto),
      [#text(weight: "bold")[#exp.title], #exp.company
       #if "location" in exp and exp.location != none { text(fill: muted, size: 9pt)[ — #exp.location] }],
      align(right, text(size: 9pt, fill: muted)[#fmt-date(exp.at("start-date")) – #fmt-date(exp.at("end-date", default: none))]))
    v(0.1em)
    for bullet in exp.at("highlights", default: ()) { [- #text(size: 9.5pt)[#bullet]] }
    v(0.3em)
  }
}

// EDUCATION
#if "education" in data and data.education.len() > 0 {
  section-heading(L.education)
  for edu in data.education {
    grid(columns: (1fr, auto),
      [#text(weight: "bold")[#edu.institution] — #edu.degree
       #if "gpa" in edu and edu.gpa != none { text(fill: muted, size: 9pt)[ (GPA: #edu.gpa)] }],
      align(right, text(size: 9pt, fill: muted)[#fmt-date(edu.at("start-date")) – #fmt-date(edu.at("end-date", default: none))]))
    for bullet in edu.at("highlights", default: ()) { [- #text(size: 9.5pt)[#bullet]] }
    v(0.2em)
  }
}

// SKILLS
#if "skills" in data and data.skills.at("groups", default: ()).len() > 0 {
  section-heading(L.skills)
  for group in data.skills.groups {
    [*#group.category:* #group.skills.join(", ")]
    v(0.1em)
  }
}

// PROJECTS
#if "projects" in data and data.projects.len() > 0 {
  section-heading(L.projects)
  for proj in data.projects {
    [#text(weight: "bold")[#proj.name]
     #if "role" in proj and proj.role != none { text(fill: muted)[ — #proj.role] }]
    v(0.1em)
    text(size: 9.5pt)[#proj.description]
    for bullet in proj.at("highlights", default: ()) { [- #text(size: 9.5pt)[#bullet]] }
    v(0.3em)
  }
}

// CUSTOM SECTIONS
#if "custom-sections" in data {
  for section in data.at("custom-sections", default: ()) {
    section-heading(section.title)
    for item in section.at("items", default: ()) {
      if "heading" in item and item.heading != none {
        if "subheading" in item and item.subheading != none {
          [#text(weight: "bold")[#item.heading] — #item.subheading]
        } else {
          [#text(weight: "bold")[#item.heading]]
        }
        v(0.1em)
      } else if "subheading" in item and item.subheading != none {
        [#text(weight: "bold")[#item.subheading]]
        v(0.1em)
      }
      if "date" in item and item.date != none {
        text(size: 9pt, fill: muted)[#item.date]
        v(0.05em)
      }
      if "body" in item and item.body != none {
        text(size: 9.5pt)[#item.body]
        v(0.1em)
      }
      for bullet in item.at("bullets", default: ()) {
        [- #text(size: 9.5pt)[#bullet]]
      }
      v(0.2em)
    }
  }
}
