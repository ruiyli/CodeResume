// Two-Column Resume Template — CodeResume
// Left sidebar (30%) + Right main area (70%)

#let lang = sys.inputs.at("lang", default: "en")
#let data-path = sys.inputs.at("data-path", default: "resume-data.json")
#let data = json(data-path)

#let L = if lang == "zh" {
  (experience: "工作经历", education: "教育背景", skills: "专业技能",
   projects: "项目经历", contact: "联系方式", present: "至今", summary: "简介")
} else {
  (experience: "Experience", education: "Education", skills: "Skills",
   projects: "Projects", contact: "Contact", present: "Present", summary: "Summary")
}

#let accent = rgb("#1e3a5f")
#let sidebar-bg = rgb("#f0f4f8")
#let muted = rgb("#6b7280")

#let body-font = if lang == "zh" {
  ("PingFang SC", "Hiragino Sans GB", "Heiti SC", "Noto Sans CJK SC")
} else {
  ("Inter", "Helvetica Neue", "Arial", "Liberation Sans")
}

#set page(paper: "a4", margin: (top: 0cm, bottom: 0cm, left: 0cm, right: 0cm))
#set text(font: body-font, size: 9pt, fill: rgb("#1f2937"))

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

#let sidebar-section(title) = {
  v(0.8em)
  text(size: 10pt, weight: "bold", fill: accent)[#title]
  v(0.3em)
  line(length: 100%, stroke: 0.5pt + accent)
  v(0.3em)
}

#let main-section(title) = {
  v(0.6em)
  text(size: 11pt, weight: "bold", fill: accent)[#title]
  v(0.2em)
  line(length: 100%, stroke: 0.5pt + rgb("#d1d5db"))
  v(0.3em)
}

#grid(
  columns: (30%, 70%),
  // ========== LEFT SIDEBAR ==========
  block(
    width: 100%,
    height: 100%,
    fill: sidebar-bg,
    inset: (x: 1cm, y: 1.2cm),
  )[
    // Photo
    #{
      let photo = data.personal.at("photo", default: none)
      if photo != none {
        align(center,
          box(radius: 50%, clip: true,
            image(photo, width: 2.8cm, height: 2.8cm, fit: "cover")))
        v(0.5em)
      }
    }
    // Name
    #text(size: 16pt, weight: "bold", fill: accent)[#data.personal.name]
    #if "name-alt" in data.personal and data.personal.at("name-alt") != none {
      v(0.1em)
      text(size: 10pt, fill: muted)[#data.personal.at("name-alt")]
    }
    #if "title" in data.personal and data.personal.title != none {
      v(0.2em)
      text(size: 9pt, fill: muted)[#data.personal.title]
    }

    // Contact
    #sidebar-section(L.contact)
    #text(size: 8.5pt)[
      #data.personal.email
      #if "phone" in data.personal and data.personal.phone != none [\ #data.personal.phone]
      #if "location" in data.personal and data.personal.location != none [\ #data.personal.location]
      #if "github" in data.personal and data.personal.github != none [\ #link(data.personal.github)[GitHub]]
      #if "linkedin" in data.personal and data.personal.linkedin != none [\ #link(data.personal.linkedin)[LinkedIn]]
      #if "website" in data.personal and data.personal.website != none [\ #link(data.personal.website)[Website]]
    ]

    // Skills
    #if "skills" in data and data.skills.at("groups", default: ()).len() > 0 {
      sidebar-section(L.skills)
      for group in data.skills.groups {
        text(size: 8.5pt, weight: "bold")[#group.category]
        v(0.1em)
        text(size: 8pt)[#group.skills.join(", ")]
        v(0.3em)
      }
    }

    // Education
    #if "education" in data and data.education.len() > 0 {
      sidebar-section(L.education)
      for edu in data.education {
        text(size: 8.5pt, weight: "bold")[#edu.institution]
        v(0.05em)
        text(size: 8pt, fill: muted)[#edu.degree]
        v(0.05em)
        text(size: 7.5pt, fill: muted)[#fmt-date(edu.at("start-date")) — #fmt-date(edu.at("end-date", default: none))]
        if "gpa" in edu and edu.gpa != none {
          v(0.05em)
          text(size: 7.5pt, fill: muted)[GPA: #edu.gpa]
        }
        v(0.3em)
      }
    }
  ],

  // ========== RIGHT MAIN AREA ==========
  block(
    width: 100%,
    inset: (x: 1cm, y: 1.2cm),
  )[
    // Summary
    #if "summary" in data and data.summary != none {
      text(size: 9pt, fill: rgb("#374151"), style: "italic")[#data.summary]
      v(0.3em)
    }

    // Experience
    #if "experience" in data and data.experience.len() > 0 {
      main-section(L.experience)
      for exp in data.experience {
        grid(columns: (1fr, auto),
          [#text(weight: "bold", size: 9.5pt)[#exp.title]
           #text(fill: muted)[ · #exp.company]],
          align(right, text(size: 8pt, fill: muted)[#fmt-date(exp.at("start-date")) — #fmt-date(exp.at("end-date", default: none))]))
        v(0.1em)
        for bullet in exp.at("highlights", default: ()) { [- #text(size: 8.5pt)[#bullet]] }
        if exp.at("technologies", default: ()).len() > 0 {
          v(0.1em)
          text(size: 7.5pt, fill: muted)[#exp.technologies.join(" · ")]
        }
        v(0.3em)
      }
    }

    // Projects
    #if "projects" in data and data.projects.len() > 0 {
      main-section(L.projects)
      for proj in data.projects {
        text(weight: "bold", size: 9.5pt)[#proj.name]
        if "role" in proj and proj.role != none { text(fill: muted, " — " + proj.role) }
        v(0.1em)
        text(size: 8.5pt)[#proj.description]
        for bullet in proj.at("highlights", default: ()) { [- #text(size: 8.5pt)[#bullet]] }
        v(0.3em)
      }
    }
  ]
)

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
