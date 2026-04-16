// ATS-Simple Resume Template — CodeResume
// Guaranteed ATS-compatible: single-column, no grids, plain text
// Best for parsing by Applicant Tracking Systems

#let lang = sys.inputs.at("lang", default: "en")
#let data-path = sys.inputs.at("data-path", default: "resume-data.json")
#let data = json(data-path)

#let L = if lang == "zh" {
  (experience: "工作经历", education: "教育背景", skills: "技能",
   projects: "项目", certifications: "证书", present: "至今", summary: "简介")
} else {
  (experience: "Experience", education: "Education", skills: "Skills",
   projects: "Projects", certifications: "Certifications", present: "Present", summary: "Summary")
}

#let body-font = if lang == "zh" {
  ("PingFang SC", "Hiragino Sans GB", "Heiti SC", "Noto Sans CJK SC")
} else {
  ("Helvetica Neue", "Inter", "Arial", "Liberation Sans")
}

// ATS-friendly: wide margins, generous spacing, no fancy formatting
#set page(paper: "a4", margin: (top: 1cm, bottom: 1cm, left: 1.5cm, right: 1.5cm))
#set text(font: body-font, size: 10pt, fill: rgb("#000000"))
#set par(leading: 0.6em)

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
  v(0.5em)
  text(size: 11pt, weight: "bold")[#title]
  v(0.3em)
}

// ============================================================================
// HEADER - Name and Contact Information
// ============================================================================
#text(size: 14pt, weight: "bold")[#data.personal.name]
#v(0.2em)

// Contact information in plain text, separated by pipes for ATS readability
#{
  let contact-parts = ()
  contact-parts.push(data.personal.email)
  if "phone" in data.personal and data.personal.phone != none {
    contact-parts.push(data.personal.phone)
  }
  if "location" in data.personal and data.personal.location != none {
    contact-parts.push(data.personal.location)
  }
  if "github" in data.personal and data.personal.github != none {
    contact-parts.push(data.personal.github)
  }
  if "linkedin" in data.personal and data.personal.linkedin != none {
    contact-parts.push(data.personal.linkedin)
  }
  text(size: 9pt)[#contact-parts.join(" | ")]
}

#v(0.4em)

// Job title if available
#if "title" in data.personal and data.personal.title != none {
  text(size: 10pt, weight: "regular")[#data.personal.title]
  v(0.3em)
}

// ============================================================================
// SUMMARY
// ============================================================================
#if "summary" in data and data.summary != none {
  section-heading(L.summary)
  text(size: 9.5pt)[#data.summary]
  v(0.3em)
}

// ============================================================================
// EXPERIENCE - Linear, no grids, plain formatting
// ============================================================================
#if "experience" in data and data.experience.len() > 0 {
  section-heading(L.experience)
  for exp in data.experience {
    text(weight: "bold", size: 10pt)[#exp.title]
    v(0.05em)
    text(size: 9.5pt)[#exp.company]

    if "location" in exp and exp.location != none {
      v(0.05em)
      text(size: 9pt)[#exp.location]
    }

    if "start-date" in exp {
      v(0.05em)
      text(size: 9pt)[#fmt-date(exp.at("start-date")) — #fmt-date(exp.at("end-date", default: none))]
    }

    if "highlights" in exp and exp.highlights.len() > 0 {
      v(0.2em)
      for bullet in exp.highlights {
        text(size: 9pt)[• #bullet]
        v(0.1em)
      }
    }

    if "technologies" in exp and exp.technologies.len() > 0 {
      v(0.05em)
      text(size: 9pt)[Technologies: #exp.technologies.join(", ")]
    }

    v(0.25em)
  }
}

// ============================================================================
// EDUCATION
// ============================================================================
#if "education" in data and data.education.len() > 0 {
  section-heading(L.education)
  for edu in data.education {
    text(weight: "bold", size: 10pt)[#edu.institution]
    v(0.05em)
    text(size: 9.5pt)[#edu.degree]

    if "gpa" in edu and edu.gpa != none {
      v(0.05em)
      text(size: 9pt)[GPA: #edu.gpa]
    }

    if "start-date" in edu {
      v(0.05em)
      text(size: 9pt)[#fmt-date(edu.at("start-date")) — #fmt-date(edu.at("end-date", default: none))]
    }

    if "highlights" in edu and edu.highlights.len() > 0 {
      v(0.1em)
      for highlight in edu.highlights {
        text(size: 9pt)[• #highlight]
        v(0.05em)
      }
    }

    v(0.2em)
  }
}

// ============================================================================
// SKILLS
// ============================================================================
#if "skills" in data and data.skills.at("groups", default: ()).len() > 0 {
  section-heading(L.skills)
  for group in data.skills.groups {
    text(size: 9.5pt)[*#group.category:* #group.skills.join(", ")]
    v(0.15em)
  }
}

// ============================================================================
// PROJECTS
// ============================================================================
#if "projects" in data and data.projects.len() > 0 {
  section-heading(L.projects)
  for proj in data.projects {
    text(weight: "bold", size: 10pt)[#proj.name]

    if "role" in proj and proj.role != none {
      v(0.05em)
      text(size: 9pt)[Role: #proj.role]
    }

    v(0.05em)
    text(size: 9.5pt)[#proj.description]

    if "url" in proj and proj.url != none {
      v(0.05em)
      text(size: 9pt)[#proj.url]
    }

    if "technologies" in proj and proj.technologies.len() > 0 {
      v(0.05em)
      text(size: 9pt)[Technologies: #proj.technologies.join(", ")]
    }

    if "highlights" in proj and proj.highlights.len() > 0 {
      v(0.1em)
      for highlight in proj.highlights {
        text(size: 9pt)[• #highlight]
        v(0.05em)
      }
    }

    v(0.2em)
  }
}

// ============================================================================
// CERTIFICATIONS
// ============================================================================
#if "certifications" in data and data.certifications.len() > 0 {
  section-heading(L.certifications)
  for cert in data.certifications {
    text(size: 9.5pt)[• #cert.name]

    if "issuer" in cert and cert.issuer != none {
      text(size: 9pt)[ — #cert.issuer]
    }

    if "date" in cert and cert.date != none {
      text(size: 9pt)[ (#fmt-date(cert.date))]
    }

    v(0.1em)
  }
}
