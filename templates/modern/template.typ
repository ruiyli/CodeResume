// =============================================================================
// Modern Tech Resume Template — CodeResume
// Supports: English (en) and Chinese (zh)
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

// --- Theme colors ------------------------------------------------------------
#let accent = rgb("#2563eb")
#let muted = rgb("#6b7280")
#let rule-color = rgb("#e5e7eb")

// --- Font selection ----------------------------------------------------------
#let body-font = if lang == "zh" {
  ("PingFang SC", "Hiragino Sans GB", "Heiti SC", "Noto Sans CJK SC", "Source Han Sans SC")
} else {
  ("Inter", "Helvetica Neue", "Arial", "Liberation Sans")
}

// --- Page setup --------------------------------------------------------------
#set page(
  paper: "a4",
  margin: (top: 1.2cm, bottom: 1.2cm, left: 1.5cm, right: 1.5cm),
)
#set text(font: body-font, size: 9.5pt, fill: rgb("#1f2937"))
#set par(leading: 0.6em, justify: false)

// --- Helper functions --------------------------------------------------------
#let section-heading(title) = {
  v(0.6em)
  text(size: 12pt, weight: "bold", fill: accent)[#title]
  v(0.2em)
  line(length: 100%, stroke: 0.5pt + rule-color)
  v(0.3em)
}

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
  text(fill: muted, size: 8.5pt)[#fmt-date(start) -- #fmt-date(end)]
}

#let pill(content) = {
  box(
    fill: rgb("#eff6ff"),
    radius: 3pt,
    inset: (x: 5pt, y: 2pt),
    text(size: 8pt, fill: accent)[#content]
  )
}

// =============================================================================
// HEADER
// =============================================================================
#align(center)[
  #{
    let photo = data.personal.at("photo", default: none)
    if photo != none {
      box(radius: 50%, clip: true,
        image(photo, width: 2.4cm, height: 2.4cm, fit: "cover"))
      v(0.3em)
    }
  }
  #text(size: 22pt, weight: "bold")[#data.personal.name]
  #{
    let na = data.personal.at("name-alt", default: none)
    if na != none {
      text(size: 14pt, fill: muted, " / " + na)
    }
  }
  #v(0.2em)
  #if "title" in data.personal and data.personal.title != none {
    text(size: 11pt, fill: muted)[#data.personal.title]
    v(0.3em)
  }
  #text(size: 8.5pt, fill: muted)[
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
  v(0.4em)
  text(size: 9pt, fill: rgb("#374151"), style: "italic")[#data.summary]
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
        #text(weight: "bold", size: 10pt)[#exp.title]
        #text(fill: muted)[ · #exp.company]
        #if "location" in exp and exp.location != none {
          text(fill: muted, size: 8.5pt)[ · #exp.location]
        }
      ],
      align(right, date-range(exp.at("start-date"), exp.at("end-date", default: none)))
    )
    v(0.15em)
    for bullet in exp.at("highlights", default: ()) {
      [- #text(size: 9pt)[#bullet]]
    }
    if exp.at("technologies", default: ()).len() > 0 {
      v(0.1em)
      for tech in exp.technologies { pill(tech); h(3pt) }
    }
    v(0.3em)
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
        #text(weight: "bold", size: 10pt)[#edu.institution]
        #text(fill: muted)[ · #edu.degree]
        #if "gpa" in edu and edu.gpa != none {
          text(fill: muted, size: 8.5pt)[ · GPA: #edu.gpa]
        }
      ],
      align(right, date-range(edu.at("start-date"), edu.at("end-date", default: none)))
    )
    for bullet in edu.at("highlights", default: ()) {
      [- #text(size: 9pt)[#bullet]]
    }
    v(0.2em)
  }
}

// =============================================================================
// SKILLS
// =============================================================================
#if "skills" in data and data.skills.at("groups", default: ()).len() > 0 {
  section-heading(L.skills)
  for group in data.skills.groups {
    [*#group.category:* #group.skills.join(", ")]
    v(0.1em)
  }
}

// =============================================================================
// PROJECTS
// =============================================================================
#if "projects" in data and data.projects.len() > 0 {
  section-heading(L.projects)
  for proj in data.projects {
    [
      #text(weight: "bold", size: 10pt)[#proj.name]
      #if "url" in proj and proj.url != none {
        text(fill: muted, size: 8pt)[ · #link(proj.url)[↗]]
      }
      #if "role" in proj and proj.role != none {
        text(fill: muted)[ · #proj.role]
      }
    ]
    v(0.1em)
    text(size: 9pt)[#proj.description]
    for bullet in proj.at("highlights", default: ()) {
      [- #text(size: 9pt)[#bullet]]
    }
    if proj.at("technologies", default: ()).len() > 0 {
      v(0.1em)
      for tech in proj.technologies { pill(tech); h(3pt) }
    }
    v(0.3em)
  }
}

// =============================================================================
// OPEN SOURCE
// =============================================================================
#if "open-source" in data and data.at("open-source").len() > 0 {
  section-heading(L.opensource)
  for oss in data.at("open-source") {
    [
      #text(weight: "bold")[#oss.project]
      #text(fill: muted)[ · #oss.role]
    ]
    v(0.1em)
    text(size: 9pt)[#oss.description]
    v(0.2em)
  }
}

// =============================================================================
// CERTIFICATIONS
// =============================================================================
#if "certifications" in data and data.certifications.len() > 0 {
  section-heading(L.certifications)
  for cert in data.certifications {
    [*#cert.name* — #cert.issuer]
    v(0.1em)
  }
}
