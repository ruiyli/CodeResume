# CodeResume P0 Improvements - Findings Report

**Date:** April 15, 2026  
**Project Path:** `/Users/albertaryli/Downloads/CV/coderesume/`

---

## Summary

CodeResume is a well-architected Rust CLI tool for generating AI-enhanced resumes with Typst templates. The project is modular (7 crates), supports bilingual output (English/Chinese), and includes AI features (Claude/OpenAI integration).

**Current State:**
- ✅ 5 professional templates with good visual design
- ✅ Core data model fully defined
- ✅ PDF generation via Typst working
- ❌ PDF text extraction stubbed out (no ATS testing)
- ❌ Not published to crates.io or Homebrew
- ⚠️ Templates use ATS-unfriendly features (columns, grids)

---

## P0-1: ATS Compatibility Check

### Current ATS Issues Found

#### Two-Column Template (`templates/two-column/template.typ`)
- **Line 60-61:** Uses `#grid(columns: (30%, 70%))` — **BREAKS ATS parsing**
- **Line 63-128:** Left sidebar in grid cell for contact/education/skills
- **Line 131-172:** Right column in grid cell for main content
- **Impact:** Two-column layout completely unreadable to ATS parsers (they read linearly)
- **Color scheme:** Sidebar has `fill: sidebar-bg` (light blue) — adds visual complexity

#### Modern Template (`templates/modern/template.typ`)
- **Line 148-158, 177-186:** Uses nested `grid()` for experience/education
- **Better than two-column:** Grid used only for inline layout (left content + right dates)
- **Line 90-97:** `pill()` helper creates boxes with fill/radius — minor ATS issue
- **Line 165, 228:** For-loop rendering tech "pills" — workaround OK for ATS

#### Classic Template (`templates/classic/template.typ`)
- **Line 91-98, 105-108:** Uses `grid()` for experience/education (left content + right dates)
- **ATS-friendly:** Single-column layout, grid only for alignment (not nesting)
- **Line 49:** Bold line separator — renders as text in PDF, ATS-readable

#### Minimal Template (`templates/minimal/template.typ`)
- **Line 53-67:** Grid for photo + header side-by-side **when photo exists**
- **Line 91-96, 104-106:** Grids for left content + right dates
- **ATS-neutral:** Grid used only for inline alignment, no nested columns

#### Academic Template (`templates/academic/template.typ`)
- **Line 92-96, 122-124:** Grid layout similar to Classic
- **Good design:** Publication section (lines 103-115) is well-structured
- **No major ATS issues**

### PDF Text Extraction

**Current Status:** Feature **stubbed but not implemented**
- **File:** `crates/cr-io/src/pdf_parse.rs` (lines 1-12)
- **TODO:** "Implement PDF text extraction using pdf-extract crate"
- **No capability to test ATS compatibility**

### Data Model - Skills

**File:** `crates/cr-core/src/resume.rs` (lines 131-141)
```rust
pub struct SkillSet {
    pub groups: Vec<SkillGroup>,  // Only flat lists
}

pub struct SkillGroup {
    pub category: String,
    pub skills: Vec<String>,
}
```
- **Missing:** Skill level/proficiency (basic/intermediate/advanced/expert)
- **Missing:** Skill ratings/years of experience
- **Current:** Only supports comma-separated skill lists

### Recommendations for P0-1

1. **Create ATS-friendly template variant**
   - "ats-simple" template: Single column, no grids, plain text formatting
   - Validates readability via text extraction
   
2. **Implement PDF text extraction**
   - Add `pdf-extract` crate dependency
   - Complete `pdf_parse.rs` implementation
   
3. **Add ATS validation command**
   - `coderesume validate <resume.yaml> --check-ats`
   - Generates PDF, extracts text, checks for ATS-breaking patterns
   
4. **Enhance skills model** (if supporting skill bars/ratings in future)
   - Add optional `level: Option<SkillLevel>` enum
   - Add optional `years: Option<f32>`
   - Avoid visual skill bars in templates initially

---

## P0-2: Publishing to crates.io / Homebrew

### Workspace Configuration

**File:** `Cargo.toml` (workspace root)
- ✅ **Version:** `0.1.0` (defined at workspace level, lines 14)
- ✅ **License:** `MIT` (line 16)
- ✅ **Authors:** `["CodeResume Contributors"]` (line 17)
- ✅ **Edition:** `2021` (line 15)

### CLI Binary Configuration

**File:** `crates/cr-cli/Cargo.toml` (lines 6-8)
```toml
[[bin]]
name = "coderesume"
path = "src/main.rs"
```
- ✅ **Binary name:** `coderesume` — good, matches project name
- ✅ **Path:** Correct to `src/main.rs`

### Missing Metadata for crates.io

**File:** `crates/cr-cli/Cargo.toml` — **INCOMPLETE**

Required/recommended fields missing:
- ❌ **description:** None (required for crates.io)
- ❌ **repository:** Should link to GitHub (https://github.com/ruiyli/CodeResume.git)
- ❌ **homepage:** Optional, e.g., GitHub repo
- ❌ **categories:** Should include `command-line-utilities`, `text-processing`, `graphics`, etc.
- ❌ **keywords:** None (e.g., resume, pdf, ai, typst)
- ❌ **readme:** Optional but recommended

**File:** `Cargo.toml` (workspace) — **Should declare package metadata**

### Cargo.lock Status

**File:** `Cargo.lock` (2527 lines)
- ✅ **Committed:** Yes, found in repository
- ✅ **Size:** 2527 lines — typical for workspace with dependencies
- **Best practice:** Lock file committed for binary crates (✓ correct for CLI)

### Current Publishing Blockers

1. **No description in cr-cli/Cargo.toml**
2. **No categories defined**
3. **No keywords defined**
4. **No repository link**

### Current State (from README.md, line 17)

The README mentions:
> "CodeResume is a CLI tool that generates beautiful, ATS-friendly PDF resumes from YAML/JSON data. It features AI-powered content optimization via Claude or OpenAI, 5 professional Typst templates, and full Chinese/English bilingual support."

This would be a good basis for the description and keywords.

### Recommendations for P0-2

1. **Update `crates/cr-cli/Cargo.toml`** with:
   ```toml
   description = "AI-powered, ATS-friendly resume generator with Typst templates"
   repository = "https://github.com/ruiyli/CodeResume"
   homepage = "https://github.com/ruiyli/CodeResume"
   categories = ["command-line-utilities", "graphics", "text-processing"]
   keywords = ["resume", "pdf", "ai", "typst", "cv"]
   readme = "../README.md"
   ```

2. **Create Homebrew formula** (`scripts/homebrew/coderesume.rb`)
   - Reference: `brew tap homebrew/crates`
   - Include binary URL and checksums
   - Declare Typst as runtime dependency

3. **Pre-publication checklist:**
   - ✅ CI/CD working (GitHub Actions already present, line 7 in README)
   - ✅ Tests passing (can add `cargo test --all` to CI)
   - ✅ Crate docs documented (`cargo doc`)
   - ✅ Examples in README ✓
   - ⚠️ Add doc comments to public API

4. **Create PUBLISHING.md** with:
   - Setup instructions
   - Release checklist
   - Version bumping strategy

---

## P0-3: Template Visual Quality

### Template Inventory

| Template | File | Lines | Style | Issues |
|----------|------|-------|-------|--------|
| Classic | `templates/classic/template.typ` | 135 | Serif, traditional | Conservative color |
| Modern | `templates/modern/template.typ` | 260 | Sans-serif, tech | Good baseline |
| Minimal | `templates/minimal/template.typ` | 130 | Ultra-clean | Limited visual hierarchy |
| Two-Column | `templates/two-column/template.typ` | 173 | Sidebar layout | Heavy design, ATS-incompatible |
| Academic | `templates/academic/template.typ` | 152 | Serif, education-first | Good structure |

### Modern Template (`templates/modern/template.typ`) — Detailed Analysis

**Visual Strengths:**
- **Line 39:** Accent color `rgb("#2563eb")` — vibrant blue, good tech appeal
- **Line 55:** Base font 9.5pt — readable
- **Line 56:** Proper line-leading (0.6em) — good spacing
- **Lines 59-65:** Section headings with underline — clean hierarchy
- **Lines 90-97:** Pill-style tech badges — visual interest via `box()` with fill

**Spacing Analysis:**
- **Header (lines 102-131):** 0.3-0.4em vertical padding — adequate
- **Sections:** 0.6em before heading, 0.2-0.3em internal — reasonable
- **Experience bullets (line 161):** 9pt font — readable
- **Tech pills (line 165):** 3pt horizontal gutter — good visual separation

**Color Scheme:**
- **Accent:** `#2563eb` (blue) — professional
- **Muted:** `#6b7280` (gray) — good contrast with body text
- **Rule:** `#e5e7eb` (light gray) — subtle separator
- **Pill background:** `#eff6ff` (very light blue) — accents without overwhelming

**Typography:**
- **Body:** Inter, Helvetica Neue, Arial — excellent tech fonts
- **Chinese:** PingFang SC, Hiragino Sans GB, Noto Sans CJK SC — proper CJK rendering
- **Sizes:** 22pt name (line 111), 12pt sections (line 61), 9.5pt body (line 55)

### Two-Column Template (`templates/two-column/template.typ`) — Detailed Analysis

**Visual Strengths:**
- **Line 16:** Accent color `rgb("#1e3a5f")` — professional dark blue
- **Line 17:** Sidebar background `rgb("#f0f4f8")` — subtle, not overwhelming
- **Line 44-50:** Section headers with underline in sidebar

**Visual Issues:**
- **Grid-based layout (line 60):** Two-column design sacrifices ATS for aesthetics
- **Sidebar width (30% / 70%):** Information density imbalanced
- **Line 74-75:** Circular photo with clipping — good visual, but ATS doesn't see it

### Design Gaps

1. **No skill bars/ratings**
   - Modern template just lists skills as pills
   - No visual indicator of proficiency level
   - Brilliant CV (reference) uses gradient bars

2. **Limited accent color customization**
   - Colors hardcoded in each template
   - No config options for theme color
   - All templates use fixed accent color

3. **No visual differentiation for prominent skills**
   - Featured/proficient skills look same as basic skills
   - No visual hierarchy for skill importance

4. **Minimal template lacks visual hierarchy**
   - `smallcaps()` for section headings (line 45)
   - Heavy on whitespace, light on visual interest
   - Good for minimalist readers, not all audiences

### Brilliant CV Reference (Standard Typst Template)

**Common patterns to learn from:**
- Skill bars with percentage/years
- Customizable accent colors via variables
- Section highlights with icons or visual markers
- Better spacing between sections
- Professional color schemes (multiple themes)
- Usage of different text weights for hierarchy

### Data Model - Styling

**Current state (from `cr-core/src/resume.rs`):**
- **No styling fields** in PersonalInfo, Experience, or Education
- **Skills data (lines 131-141):** No proficiency level
- **No accent color configuration** anywhere

### Recommendations for P0-3

1. **Enhance Modern template** (already good baseline)
   - Increase spacing between sections (currently tight)
   - Add subtle icons/bullets for experience items
   - Consider light card backgrounds for experience blocks
   - Better typography hierarchy with font weights

2. **Create new "Brilliant" template variant**
   - Add skill bars with proficiency indicators
   - Support customizable accent color
   - Better visual separation of sections
   - Professional color schemes (Dark/Light mode)

3. **Enhance data model** to support styling
   ```rust
   // Add to Resume or new StyleConfig
   pub struct TemplateStyle {
       pub accent_color: Option<String>,     // #RRGGBB
       pub theme: Option<String>,             // "light", "dark"
       pub show_skill_bars: Option<bool>,
       pub show_icons: Option<bool>,
   }
   
   // Add to SkillGroup
   pub struct SkillGroup {
       pub category: String,
       pub skills: Vec<SkillWithLevel>,
   }
   
   pub struct SkillWithLevel {
       pub name: String,
       pub level: Option<u8>,  // 0-100 percent
   }
   ```

4. **Update minimal template** for better hierarchy
   - Add subtle color accents
   - Increase section spacing slightly
   - Consider adding very light background shading
   - Keep ultra-clean aesthetic but improve readability

5. **Document template philosophy** in each template file
   - Line 1-3 comments explaining design intent
   - Example config options
   - Tips for customization

---

## Implementation Priority

### Quick Wins (1-2 days each)

1. **P0-2.1:** Add metadata to `cr-cli/Cargo.toml`
   - Description, repository, categories, keywords
   - Enable `cargo publish --dry-run` check

2. **P0-3.1:** Document Modern template improvements
   - Identify spacing/typography tweaks
   - Create design spec for enhancements

### Medium Effort (2-4 days each)

3. **P0-1.1:** Implement PDF text extraction
   - Add `pdf-extract` crate to `cr-io`
   - Complete `pdf_parse.rs`

4. **P0-1.2:** Create ATS validation command
   - New command: `validate`
   - Extract text from generated PDF
   - Check for column/grid patterns

5. **P0-3.2:** Enhance Modern template implementation
   - Update spacing/colors
   - Add better section hierarchy
   - Test visual output

### Longer Term (4-7 days each)

6. **P0-2.2:** Create Homebrew formula
   - Package binary
   - Create tap repository structure
   - Test installation

7. **P0-1.3:** Create ATS-simple template
   - Single column, no grids
   - Plain text friendly
   - Validation tests

8. **P0-3.3:** Create Brilliant template variant
   - Skill bars implementation
   - Customizable colors
   - Multiple themes

---

## Key Files Summary

| File | Purpose | Key Lines |
|------|---------|-----------|
| `Cargo.toml` | Workspace config | 1-61 |
| `crates/cr-cli/Cargo.toml` | CLI binary config (NEEDS UPDATES) | 1-24 |
| `crates/cr-core/src/resume.rs` | Data model | 5-295 |
| `crates/cr-core/src/template.rs` | Template metadata | 5-103 |
| `crates/cr-config/src/model.rs` | App configuration | 1-147 |
| `crates/cr-io/src/pdf_parse.rs` | PDF text extraction (STUBBED) | 1-12 |
| `templates/two-column/template.typ` | Two-column template (ATS-incompatible) | 60-172 |
| `templates/modern/template.typ` | Modern template (best baseline) | 1-260 |
| `README.md` | Project overview | 1-249 |

---

## Architecture Insights

```
cr-cli (entry point)
  └─ cr-service (business logic)
     ├─ cr-ai (AI integration)
     ├─ cr-io (file I/O + PDF parsing)
     ├─ cr-render (Typst engine)
     └─ cr-core (data types)
  └─ cr-config (configuration)
```

**Strengths:**
- ✅ Clear separation of concerns
- ✅ Modular design allows independent testing
- ✅ Abstracted template registry
- ✅ Flexible data format (YAML/JSON/Markdown)

**Growth Points:**
- ⚠️ PDF text extraction not yet complete
- ⚠️ No styling/theme configuration system
- ⚠️ Template customization limited (hardcoded colors)
- ⚠️ No ATS validation framework

---

## Findings End
