# CodeResume Template Customization Guide

**Date:** April 16, 2026  
**Version:** 1.0

---

## Overview

CodeResume provides 7 professional templates for different use cases and preferences:

| Template | Best For | Visual Style | Features |
|----------|----------|--------------|----------|
| **Modern** | Tech companies, startups | Clean blue accents | Tech badges, professional |
| **Brilliant** | Advanced users, designers | Customizable colors, skill bars | Premium design, visual flair |
| **Classic** | Enterprise, formal roles | Traditional serif | Timeless, conservative |
| **Academic** | Researchers, academics | Publication-focused | GPA display, publication section |
| **Minimal** | Senior engineers, minimalists | Ultra-clean | Maximum whitespace, scannable |
| **Two-Column** | Portfolios, agencies | Sidebar layout | Visual impact (not ATS-friendly) |
| **ATS-Simple** | Corporate, ATS-critical | Plain text | Guaranteed ATS compatibility |

---

## Template Selection

### By Industry

**Technology & Startups:**
- Recommended: **Modern** (default), **Brilliant**
- Why: Tech badge display, modern colors, professional design
- Command: `coderesume generate resume.yaml --template modern`

**Finance, Law, Enterprise:**
- Recommended: **ATS-Simple**, **Classic**
- Why: ATS-safe, formal, professional appearance
- Command: `coderesume generate resume.yaml --template ats-simple`

**Academia & Research:**
- Recommended: **Academic**
- Why: Publication section, education-first ordering, GPA support
- Command: `coderesume generate resume.yaml --template academic`

**Design & Creative Industries:**
- Recommended: **Brilliant**, **Two-Column**
- Why: Visual design, color customization, sidebar layout
- Command: `coderesume generate resume.yaml --template brilliant`

**Maximum ATS Compatibility:**
- Recommended: **ATS-Simple**
- Why: Single-column, plain text, linear reading order
- Command: `coderesume generate resume.yaml --template ats-simple --check-ats`

---

## Modern Template — Tech-Forward Professional

### Features
- ✅ Clean blue accent color (#2563eb)
- ✅ Tech skill "pills" (visual badges)
- ✅ Professional gray for hierarchy
- ✅ Good spacing and readability
- ✅ ATS-compatible layout

### Usage
```bash
coderesume generate my-resume.yaml --template modern --format pdf
```

### When to Use
- Default choice for tech professionals
- Balanced between visual appeal and ATS compatibility
- Works well for: 1-2 page resumes, any industry

---

## Brilliant Template — Premium Design

### Features
- ✨ **Customizable color themes** — change primary accent color
- ✨ **Skill rating bars** — visual proficiency indicators
- ✨ **Advanced visual design** — premium appearance
- ✨ **Accent pill design** — for technologies and keywords
- ✨ **Color-coded section headers** — visual hierarchy
- ⚠️ **Note:** Primarily designed for visual impact, not ATS-optimized

### Visual Elements

**Skill Rating System:**
```
Expert:       ●●●●
Advanced:     ●●●○
Intermediate: ●●○○
Learning:     ●○○○
```

**Color Scheme (Customizable):**
```
Primary:   #2563eb (Blue) — Professional, tech-focused
Light:     #dbeafe (Light Blue) — Backgrounds
Dark:      #1e40af (Dark Blue) — Darker accents
Text:      #1f2937 (Near black) — Primary text
Secondary: #6b7280 (Gray) — Secondary text
Expert:    #059669 (Green) — Skill level indicator
Advanced:  #0891b2 (Cyan) — Skill level indicator
```

### Customization

The Brilliant template uses a theme configuration system. While direct theme customization is built into the template, users can modify the color variables by editing the YAML directly (advanced usage).

### Usage
```bash
# Generate with Brilliant template
coderesume generate my-resume.yaml --template brilliant --format pdf

# Validate appearance
coderesume validate my-resume.yaml --template brilliant
```

### When to Use
- When visual impact is important
- Design-heavy industries
- Online portfolios and web-based CVs
- Candidates wanting to stand out visually
- **Not recommended:** Industries with strict ATS scanning

---

## Classic Template — Traditional Formal

### Features
- ✅ Serif fonts (Georgia, Times New Roman)
- ✅ Black & gray monochromatic design
- ✅ ALL CAPS section headers
- ✅ Professional, timeless appearance
- ✅ ATS-compatible layout

### Usage
```bash
coderesume generate my-resume.yaml --template classic --format pdf
```

### When to Use
- Enterprise, banking, finance roles
- Government positions
- Traditional industries (law, consulting)
- When formal appearance is critical

---

## Academic Template — Research & Education Focus

### Features
- ✅ Purple accent color (#7c3aed)
- ✅ Publication section (prominent)
- ✅ Education listed first (academic convention)
- ✅ GPA display with styling
- ✅ Research-friendly section organization
- ✅ ATS-compatible layout

### Special Sections
1. **Education** — Appears first (before Experience)
2. **Publications** — Full citation-style formatting
3. **Research Projects** — Separate from work projects

### Usage
```bash
coderesume generate my-resume.yaml --template academic --format pdf
```

### When to Use
- Academic CVs and research portfolios
- PhD candidates and postdocs
- Faculty positions
- Research institutions
- When publication history is key selling point

---

## Minimal Template — Ultra-Clean & Scannable

### Features
- ✅ Maximizes whitespace
- ✅ Bold section headers (improved scanning)
- ✅ No colors or decorations
- ✅ Compact font sizing
- ✅ Professional restraint
- ✅ ATS-compatible layout

### Styling
- Font size: 9pt (compact)
- Line leading: 0.55em (tight spacing)
- No accent colors (monochromatic)
- Serif font option available

### Usage
```bash
coderesume generate my-resume.yaml --template minimal --format pdf
```

### When to Use
- Senior engineers and architects
- Minimalist personal brand
- When content matters more than design
- Maximum information density needed
- ATS-critical roles

---

## Two-Column Template — Sidebar Design

### Features
- ⭐ Beautiful sidebar layout (30% left, 70% right)
- ⭐ Sidebar contains: photo, contact, skills
- ⭐ Main area contains: experience, education, projects
- ⭐ Professional color scheme (navy + light blue)
- ❌ **WARNING:** Not ATS-compatible (grid layout breaks parsing)

### Visual Design
- Left sidebar: Light blue background (#f0f4f8)
- Navy headers (#1e3a5f)
- Excellent for visual impact

### Usage
```bash
coderesume generate my-resume.yaml --template two-column --format pdf
```

### When to Use
- ✓ Portfolios and personal websites
- ✓ Design and creative roles
- ✓ Agencies and studios
- ✓ When ATS is NOT a concern
- ✗ NOT for: Corporate applications with ATS screening

### Why Not ATS-Compatible
The grid layout causes ATS systems to parse:
1. Photo and sidebar content FIRST
2. Main content SECOND

This breaks the logical resume flow in applicant tracking systems.

---

## ATS-Simple Template — Guaranteed Compatibility

### Features
- ✅ Single-column layout (no grids)
- ✅ Plain text formatting (no colors)
- ✅ Linear reading order
- ✅ Maximum ATS compatibility
- ✅ 100% text-extractable
- ✅ All sections rendered clearly

### Design Approach
- No grid layouts
- No fancy formatting
- Plain bullet points
- Simple text hierarchy
- All resume data in natural flow

### Usage
```bash
coderesume generate my-resume.yaml --template ats-simple --format pdf

# Validate ATS compatibility
coderesume validate my-resume.yaml --template ats-simple --check-ats
```

### When to Use
- **Recommended for:** Corporate, government, enterprise applications
- Banking, pharmaceuticals, healthcare
- Large companies with ATS screening
- When ATS compatibility is critical
- Unknown company, assume they use ATS
- Job postings that don't specify

### ATS Validation
Test your resume with the validate command:

```bash
# Check ATS compatibility for all templates
coderesume validate my-resume.yaml --check-ats

# Check specific template
coderesume validate my-resume.yaml --template ats-simple --check-ats
```

---

## Template Comparison Matrix

### Visual Design Quality

| Aspect | Modern | Brilliant | Classic | Academic | Minimal | Two-Col | ATS-Simple |
|--------|--------|-----------|---------|----------|---------|---------|-----------|
| Visual Appeal | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| Readability | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| ATS Safe | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ✗ | ⭐⭐⭐⭐⭐ |
| Scanability | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| Modern Feel | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| Professional | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |

### Content Support

| Feature | Modern | Brilliant | Classic | Academic | Minimal | Two-Col | ATS-Simple |
|---------|--------|-----------|---------|----------|---------|---------|-----------|
| Experience | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Education | ✓ | ✓ | ✓ | ✓ First | ✓ | ✓ | ✓ |
| Skills | ✓ | ✓ Rated | ✓ | ✓ | ✓ | ✓ | ✓ |
| Projects | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Publications | ✓ | ✓ | ✗ | ✓ Special | ✗ | ✗ | ✓ |
| Certifications | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ | ✓ |
| Tech Badges | ✓ Pill | ✓ Pill | ✗ | ✗ | ✗ | ✗ | List |
| Photo | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | Optional |

---

## Advanced: Customization Tips

### Choosing Template by Resume Length

**One Page:**
- Best: Modern, Minimal, ATS-Simple
- Why: Compact spacing, efficient layout

**1-2 Pages:**
- Best: Modern, Brilliant, Academic
- Why: Good spacing, room for content

**2+ Pages:**
- Best: Classic, Academic, Two-Column
- Why: Designed for longer content

### Recommended Workflow

1. **First Pass:** Use **Modern** template
   - Good balance of visual appeal and ATS compatibility
   - Default choice for most professionals

2. **Review Output:**
   - Does it fit your content well?
   - Does it look professional in your industry?

3. **Consider Alternatives:**
   - Too formal for tech? Try **Brilliant**
   - Very formal industry? Try **Classic**
   - Academic work? Use **Academic**
   - Publications matter? Use **Academic**
   - Maximum compatibility needed? Use **ATS-Simple**

4. **Validate ATS Compatibility:**
   ```bash
   coderesume validate my-resume.yaml --check-ats
   ```

5. **Test with Recruiters:**
   - If possible, ask recruiters to test with their ATS
   - Compare text extraction results

---

## Generation Commands Reference

```bash
# Generate with default template (Modern)
coderesume generate resume.yaml

# Generate with specific template
coderesume generate resume.yaml --template brilliant
coderesume generate resume.yaml --template ats-simple
coderesume generate resume.yaml --template academic

# Generate both PDF and Markdown
coderesume generate resume.yaml --template modern --format pdf --format markdown

# Generate and validate ATS
coderesume validate resume.yaml --template ats-simple --check-ats

# List all available templates
coderesume templates
```

---

## FAQ

**Q: Which template should I use by default?**
A: **Modern**. It's the best balance of visual appeal, readability, and ATS compatibility for most professionals.

**Q: Is the Brilliant template ATS-safe?**
A: No, it prioritizes visual design over ATS compatibility. Use Modern or ATS-Simple for ATS-critical applications.

**Q: Can I customize colors in templates?**
A: The Brilliant template has a built-in theme system. For other templates, you would need to edit the Typst source directly (advanced).

**Q: Which template is best for a 2-page resume?**
A: Classic or Academic templates handle longer content well with good spacing.

**Q: Should I worry about ATS compatibility?**
A: Yes, if applying to large companies (100+ employees) or any company not mentioned in your network. Start with Modern or ATS-Simple.

**Q: Can I use Two-Column if I know the company doesn't use ATS?**
A: Yes! Two-Column template creates beautiful visual designs perfect for portfolios and companies without ATS screening.

**Q: How do I check if my resume is ATS-compatible?**
A: Use the validate command: `coderesume validate resume.yaml --check-ats`

---

## Template Roadmap

### Planned Enhancements
- Color customization system for all templates (in-progress)
- Skill rating system for all templates (in-progress)
- Additional premium templates (coming soon)
- Dark mode support (planned)
- Responsive mobile view (planned)

---

## Support & Feedback

For template-related questions or feedback:
- GitHub Issues: https://github.com/ruiyli/CodeResume/issues
- Documentation: https://github.com/ruiyli/CodeResume

---

**Last Updated:** April 16, 2026  
**CodeResume Version:** 0.1.0

