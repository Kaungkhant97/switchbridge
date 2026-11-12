# C4 and data-flow diagram practices for SwitchBridge architecture planning

Research date and source access date: 2026-11-12. This is a research note for architecture planning. It contains **no final diagrams**.

Each claim below is labeled by its basis:

- **[Sx]**: checked on 2026-11-12 against the primary source listed in [Sources](#sources). Short quotations appear in quotation marks, and each page contributes at most 25 quoted words in total. Everything else is paraphrase.
- **Inference**: a recommendation or reasoning by the author. No source states it directly.
- **Repo**: something observed in this worktree at `ce8aeac`, with file and line given.

Nothing below comes from unverified recollection.

## 1. What the primary sources establish

### 1.1 C4 abstractions

- **Software system** [S2]: the top abstraction, something that delivers value to its users. A useful test is that one development team builds, owns and is responsible for it. Domains, bounded contexts, capabilities and teams are usually not software systems.
- **Container** [S3]: an application or a data store. It is "a runtime boundary around some code that is being executed or some data that is being stored". It is explicitly not a Docker container. The examples include a standalone server-side console application, a batch process and a shell script. Libraries, assemblies and modules organise code inside an application and are typically not containers.
- **Component** [S4]: related functionality grouped behind a well-defined interface. Components are not separately deployable. All components in a container share one process space. Packaging units such as JARs or namespaces are not components by default.
- **Hierarchy** [S1]: a software system contains containers, containers contain components, and code elements implement components.

### 1.2 C4 diagram types relevant here

- **System context** [S5]: the scope is one software system, with directly connected people and external systems as supporting elements. Detail is deliberately low; the focus is on people and systems rather than technologies. The model recommends this view for every team.
- **Container** [S6]: the scope is one software system, showing the containers inside it. The model recommends this view for every team. Deployment concerns such as clustering or failover belong in deployment diagrams.
- **Dynamic** [S7]: the scope is a feature, story or use case. It shows runtime collaboration with numbered interactions. The collaboration and sequence styles carry the same information.
- **Deployment** [S8]: the scope is one deployment environment. Its primary elements are deployment nodes and software-system or container instances. Deployment nodes may nest.

### 1.3 C4 notation and review checklist

- **Notation** [S9]: C4 is notation independent. Its requirements:
  - Each diagram has a title (type and scope) and a key or legend.
  - Each element states its type and a short description. Each container and component names its technology.
  - Lines are unidirectional, labelled consistently with their direction and intent, and specific: avoid labels like "Uses". Relationships between containers name their protocol.
  - Colour is free but must be consistent, and must account for printing and colour blindness.
- **Review checklist** [S10]:
  - *General*: title, diagram type, scope, legend.
  - *Elements*: name, abstraction type, responsibility, technology, and the meaning of acronyms, colours, shapes, icons, borders and sizes.
  - *Relationships*: an intent label, a label that matches the direction, technology/protocol, and the meaning of acronyms, colours, arrow heads and line styles.
- **FAQ** [S11]: for libraries, frameworks and SDKs, the FAQ suggests UML may suit better. I found no FAQ entry on CLIs or batch tools on 2026-11-12.

### 1.4 DFD elements and trust boundaries (Microsoft threat modeling)

| Element | Shape | What the source says it represents | Context the source asks for |
| --- | --- | --- | --- |
| Process [S13] | Circle | Activities that modify or redirect input to outputs | Language, permission level, isolation/sandbox, who can supply input, validation, authN, authZ |
| Data store [S14] | Parallel lines | Data held temporarily or permanently | Type, function, permissions, encryption/signatures |
| External entity [S15] | Square | A process, store or system outside your direct control, which you cannot modify | Internal or external source, type (human/service), authN, authZ |
| Data flow [S16] | Directional arrow | Data movement between elements | Data description, protocol, enumerated flow sequence, payload type, controls, authN, authZ; usually include responses |
| Trust boundary [S17] | Dotted line or box | Data crossing between different trust zones; lines suit large environments such as the internet, boxes smaller ones such as sandboxes | A description of the boundary |

Rules from the same module:

- Two data stores communicate only through a process [S13][S14].
- The store and external-entity units [S14][S15] tell modellers to check that a data store or external entity is present as a starting point of the data flow. That is a completeness check, not an exhaustive rule about where every arrow begins. The same module's examples include process→store flows (a process requesting to add an entry to a data store [S16]) and process→process interaction (processes placed between processes [S13]). The no-direct-store→store rule above still applies.
- A process can be a stub that expands into a lower-level DFD [S13].
- Changing trust zones attract attackers and need careful design [S17].
- Microsoft's own internal trust-zone requirements are not published [S17].

The Threat Modeling Tool overview [S18] adds:

- Its stencil categories:
  - process: applications, threads, VMs;
  - external interactor;
  - data store: including configuration files;
  - data flow: including binary and named pipe;
  - trust boundary: including machine, sandbox and user/kernel mode.
- Elements can be marked out of scope with a recorded reason. The page discourages doing so.

### 1.5 Mermaid (only if Mermaid is chosen)

- **Mermaid C4** [S19]: on 2026-11-12 the page showed Mermaid 12.3.0 and five C4 diagram kinds (context, container, component, dynamic, deployment). That page does **not** carry an "experimental" warning, and this note makes no such claim. It documents these limits:
  - layout depends on statement order rather than a fully automatic algorithm;
  - styling is fixed;
  - sprites, tags, links and legends are unsupported;
  - several tag and line-style helpers are not implemented.
- **Flowchart** [S20] syntax covers what a DFD needs:
  - circle `id((text))` and cylinder `id[(text)]`;
  - `subgraph id [title] … end`, with edges allowed to target a subgraph;
  - dotted links `A-.->B` and edge labels `A-->|text|B`;
  - `classDef`/`class`/`:::` styling;
  - `%%` comments.
- **Accessibility** [S21]: every Mermaid diagram type accepts `accTitle:` and `accDescr:` / `accDescr { … }`. These render to SVG `<title>`/`<desc>` with ARIA references.

## 2. Mapping C4 to DFD views (inference)

C4 static views show what exists and who owns it. DFDs show what data moves, where it rests, and where trust changes. The sources define no mapping between them, so the following is a planning aid.

| C4 concept | Closest DFD concept | Caveat |
| --- | --- | --- |
| Person / external software system [S5] | External entity [S15] | Close match. |
| Container that runs code [S3] | Process [S13] | A container is a type; a DFD process of *our* executable is a running instance of it. *In the model chosen for SwitchBridge*, the two invocations of the binary are two processes and are shown as container instances [S8] or dynamic-diagram participants [S7], not as two containers. This is a modelling decision for this repository, not a universal C4 rule. C4 defines containers as runtime boundaries [S3] and does not say a shared binary must always be one container. Another system whose roles from one binary run as separately operated applications could reasonably model them differently. **In this model, only processes of SwitchBridge's own executable map to its C4 container.** OS tools (`system_profiler`, `sysctl`, `sw_vers`), `ffmpeg`/`ffplay`, and CI runners or Actions services are processes outside the system. Show them as external entities [S15] or as external processes explicitly labelled outside SwitchBridge, with no mapping to a SwitchBridge container. |
| Data-store container [S3] | Data store [S14] | Whether a file counts as a C4 container is a modelling choice. State it in the legend. Treating caller-chosen files as data stores outside the executable is defensible. |
| Component [S4] | Sub-process in a zoomed DFD stub [S13] | Components share one process space, so prefer drawing DFDs at process granularity. |
| Relationship [S9] | Data flow [S16] | A C4 arrow shows dependency or intent; a DFD arrow shows data movement and usually its response. Do not use one arrow set for both. |
| Dynamic numbering [S7] | Flow sequence [S16] | Reuse the same step numbers across both views. |
| Deployment node [S8] | Trust boundary [S17] (sometimes) | A deployment node, a file, or an OS-process edge is **not automatically** a trust-zone change. Draw a trust boundary only where a stated change in trust exists [S17]. |

## 3. SwitchBridge-specific guidance (Repo + inference)

### Repo facts

- One binary, `switchbridge`, dispatches four subcommands: `send`, `receive`, `usb-snapshot` and `usb-diff` (`src/main.rs:19-42`).
- **Timeouts.** A 10-second constant (`src/main.rs:9`) applies to:
  - the sender's connect (`connect_timeout`, `:34`) and write timeout (`:35`);
  - the receiver's read timeout on the accepted socket (`:56`).

  The receiver's `listener.accept()` (`:49`) has **no timeout**. It waits until a connection arrives or the process is interrupted.
- Addresses are numeric loopback only. The receiver takes one connection and refuses an existing output path (`README.md`, help text at `src/main.rs:10`).
- **`usb-snapshot PATH`** runs these programs (`src/usb.rs:9,38,52-56`):
  - `/usr/sbin/system_profiler` twice: once to detect the data type, once to collect the inventory;
  - `/usr/sbin/sysctl -n hw.model`;
  - `/usr/bin/sw_vers -productVersion`.

  It redacts supported private values and writes them to a **caller-chosen path**. It rejects stdout over 16 MiB only *after* collecting it.
- **`usb-diff BEFORE AFTER`** reads two caller-chosen files and prints a redacted diff to **stdout**. Any file capture of that output is the user's own shell redirection.
- **`.evidence/`** is only a gitignored convention for raw material, documented in `README.md:68` and `.gitignore:2`. The code does not write there automatically. No source anchor shows an automatic raw-evidence store.
- Live Switch 2 capture is UNVERIFIED (`README.md`; `docs/research/stock-switch2-usb-feasibility.md`).

### Inferences, grounded in [S3][S4][S7][S8][S15][S17]

Items 1–2 are modelling choices recommended for SwitchBridge as it exists at `ce8aeac`. They are not general C4 law for every shared binary.

1. **Chosen model: the one SwitchBridge CLI binary is one C4 container**, a console application [S3]. The reasons: one std-only executable, launched ad hoc by one operator on one Mac, with no separately deployed or operated roles (`README.md`, `src/main.rs:19-42`). Its subcommands are entry points of that one runtime artifact. In this model they are not drawn as containers or services. Revisit the choice if a role later becomes a separately deployed or long-running application.
2. **In this model, send/receive appear only as instances.** Show them as two processes of the same container in the dataflow, or in the dynamic or deployment view [S7][S8]. Do not draw them as client and server containers in SwitchBridge's views.
3. **Do not invent infrastructure.** There is no daemon, queue, database, cloud service or Docker. A small container view is acceptable. If it adds nothing beyond the context view, say so.
4. **External entities and processes:**
   - the operator;
   - `system_profiler`, `sysctl` and `sw_vers`;
   - `ffmpeg` (test-input generator) and `ffplay` (post-completion playback), both outside the executable;
   - in the CI view, GitHub Actions runners and artifact storage.
5. **Data stores** [S14]:
   - the input H.264 file;
   - the receiver output file, which is created exclusively, removed on handled errors, and may remain partial after a kill;
   - the snapshot file at the caller's path.

   Diff output is a stdout flow, not a store, unless the user redirects it.
6. **Boundaries.** Separate assessed validation boundaries from trust-zone changes:
   - *Assessed validation boundaries*, to describe in context tables. Do not draw them as trust boundaries by default:
     - OS-tool stdout entering the parser and redaction;
     - user-supplied paths and file content;
     - snapshot/diff inputs;
     - the redaction step itself;
     - the raw-versus-redacted privacy distinction.

     These are points where SwitchBridge validates or transforms data. On their own they do not show a change of security zone.
   - *Candidate trust-zone changes*, each requiring a written justification [S17]:
     - **Loopback listener:** any local process can connect, so state whether local peers are in or out of scope.
     - **CI view:** the GitHub-hosted runner or artifact storage, then download to a contributor's machine.

     Draw these only if the justification holds.
   - **Console USB: no flow in implemented views.** The Switch 2 belongs only in a visibly isolated UNVERIFIED callout. It has no data-flow arrow in any implemented view. Record why it is out of scope rather than omitting it silently [S18].
7. **Flow labels** [S16] name the payload and the mechanism. Examples:
   - "SWB1 v1 packets (24-byte header, ≤1 MiB H.264 chunk) over TCP loopback";
   - "USB inventory text (type detected at runtime) via subprocess stdout".

   Number each step. State explicitly that SWB1 has **no** application-level response or ACK. Do not label chunk timestamps as frame timing.

## 4. Recommendations for architecture planning (inference unless cited)

1. **Use a small view set.**
   - Baseline: context plus container [S5][S6].
   - Runtime: dataflow views showing instances [S7][S13]–[S17].
   - Add a deployment view only if it adds information [S8].
   - Add a component diagram only for real interface-backed groupings [S4].
2. **Keep C4 and DFD separate but cross-referenced.** Use shared names and shared step numbers [S7][S16]. In SwitchBridge's chosen model, only processes of its own executable map to its container (§2–§3).
3. **Encode status in the notation.** Use legend-explained, consistent styles for implemented, synthetic/prerecorded, manual, and UNVERIFIED [S9].
4. **Give every element and flow its context.** C4 needs type, description and technology [S9]. DFD context tables need the §1.4 fields, and unknown OS permissions are stated as unknown.
5. **Mermaid choices.**
   - Ordinary flowcharts are adequate and portable for DFDs [S20].
   - Mermaid C4 is documented, but it lacks legend support [S19] while C4 requires a legend [S9][S10]. Either use it with an adjacent Markdown legend, or use flowcharts with explicit C4 type labels.
   - Add `accTitle`/`accDescr` to every diagram [S21].
   - Record the Mermaid version actually used by the target viewer.

## 5. Review checklist for SwitchBridge architecture diagrams

Use this checklist for later independent semantic and visual review. † marks items taken from the C4 checklist or notation [S9][S10]. ‡ marks items taken from the Microsoft DFD module [S13]–[S17]. Unmarked items are SwitchBridge-specific inferences.

**General**
- [ ] Every diagram has a title (type and scope) † and a legend for shapes, colours, borders, line styles and arrow heads †.
- [ ] Acronyms (SWB1, DFD, USB, MTP) are expanded †.
- [ ] Mermaid diagrams have `accTitle`/`accDescr`. A Mermaid C4 diagram has an adjacent legend.

**Abstraction correctness**
- [ ] Every element's C4 type is explicit and correct †.
- [ ] No subcommand is drawn as a container, service or server. No fictitious infrastructure appears.
- [ ] Rust modules are not presented as C4 components unless an interface-backed grouping is justified [S3][S4].
- [ ] Consistent with the chosen SwitchBridge model (§3), two runs of the binary are shown as instances or participants [S7][S8].
- [ ] In that model, only own-executable processes map to the container. OS tools, ffmpeg/ffplay and CI infrastructure are labelled outside the system.

**Elements**
- [ ] Each element has a name, a responsibility and (for containers/components) a technology †.
- [ ] Process context: language, permissions, isolation, input source and validation. Unknown values are marked unknown ‡.
- [ ] Store context: type, function, permissions and controls ‡.
- [ ] External-entity context: source and type ‡.
- [ ] Snapshot paths are caller-chosen. Diff output goes to stdout. No automatic raw-evidence store is depicted.

**Relationships / flows**
- [ ] Every arrow is unidirectional, has a specific label that matches its direction †, and names its protocol or mechanism †.
- [ ] Flows are numbered ‡. Responses are drawn where they exist. SWB1's lack of an ACK is stated ‡.
- [ ] No flow runs store-to-store ‡.
- [ ] Each scenario has at least one store or external entity as a starting point ‡ (completeness check). Process→store and process→process flows are allowed [S13][S16].
- [ ] Timeout depiction: connect and connected I/O are 10 s; listener accept is unbounded.
- [ ] No flow claims frame timing, live capture or console USB video. The console appears only in an isolated UNVERIFIED callout.

**Boundaries**
- [ ] Every drawn trust boundary has a description ‡ and a justified change in trust.
- [ ] Validation/privacy/OS-process edges are not presented as trust-zone changes without that justification.
- [ ] The local-peer scope of the loopback listener is stated.
- [ ] Out-of-scope elements carry a reason [S18].

**Consistency**
- [ ] Names, styles and statuses are identical across views †.
- [ ] Everything agrees with the pinned source, `README.md` and the feasibility report.

## 6. Limitations

- I read pages through a fetch tool that summarises. Quoted phrases came back as verbatim text, but full page text was not archived. Re-open the URLs before quoting them in final deliverables.
- The sources cover neither Rust crates nor CLI subcommands. §2–§3 apply [S3][S4] by inference.
- No STRIDE analysis was done.

## 7. Independent review of the architecture plan (read-only)

**Reviewed file:** `.worktrees/architecture-docs/docs/superpowers/plans/2026-11-12-architecture-documentation.md`, revised version, 150 lines.

- SHA-256: `49cf74fcc71ae86c4ebcddbaa9cb318a2acfa41d9cdf84863c96466558c02500`.
- Read on 2026-11-12.

**Spot-checked against `ce8aeac` (all matched the plan):**

- `src/usb.rs:9,38,55-56`;
- `src/main.rs:9,34,35,49,56`;
- `docs/macos-playback.md:84`;
- `.github/workflows/macos.yml:48,81,85`.

**Resolved in the revision:**

- numbered flows and the no-ACK statement (line 57);
- context tables, with unknown permissions left explicit (line 57);
- trust boundaries only with a stated change (line 57);
- accessibility and the Mermaid version record (line 106);
- the file-container modelling choice explained (line 101);
- the timeout and accept distinction (line 67);
- the "no experimental claim" correction (lines 104, 118).

**Remaining findings:**

- **F1 (MEDIUM, open): modules called components.** Line 62 still says `main`, `transport`, `framing` and `usb` "are components/modules within the application". Per [S3][S4], code-organisation modules are not C4 components by default; components need a well-defined interface. Suggested fix: "Rust modules (code organisation inside the one container); no C4 component claim is made." Alternatively, justify each module's interface.
- **F2 (LOW): acceptance rubric not named.** Acceptance (line ~150) should name the C4 checklist [S10] and §5 of this note as the semantic review rubric, so render receipts cannot stand in for notation completeness. Line 102 cites the checklist only as research input.
- **F3 (LOW): external processes not separated from the container in the DFD contract.** Line 57 says external OS commands remain outside the executable. Add a rule to the DFD contract: only processes of the SwitchBridge executable map to the C4 container; OS-tool, ffmpeg/ffplay and CI processes are drawn as external.
- **F4 (INFO): Archify is outside this review.** The Archify commands, the 9-check criterion and the 12-node limit are outside the primary sources I verified. This review does not confirm them.

### Recheck and dispositions

**Rechecked file:** the same plan, revised version, 154 lines.

- SHA-256: `e4269db3eaf8add215f6a298d90aa5ef31e05991c685be4a3ec286ff7f1c5bbb`.
- Read on 2026-11-12.

| Finding | Disposition | Evidence in revised plan |
| --- | --- | --- |
| F1 modules vs components | **RESOLVED** | Line 66 now says "Rust code-organization modules, with no C4 component claim". This matches [S3][S4]. |
| F2 acceptance rubric | **RESOLVED** | Line 154 names the official C4 review checklist and this note's DFD checklist as the semantic rubric, separate from Archify layout checks. It also lists type/responsibility/technology, direction labels/protocols and legend meanings [S9][S10]. |
| F3 external processes vs container | **RESOLVED (in substance)** | Line 57: OS commands and playback tools stay outside the executable. Line 69: CI infrastructure boundary is distinct. Line 61: Actions artifacts are externally obtained bytes. Optional wording: in line 57, also name CI runner processes as external, not instances of the SwitchBridge container. Non-blocking. |
| F4 Archify outside sources | **ACKNOWLEDGED (INFO)** | Not verified by this review. No change required. |
| Earlier: file container modelling choice | **RESOLVED** | Line 59 adds a README legend statement: caller-selected files are external artifacts in C4 and stores in the DFD. This is a modelling choice; C4 still permits owned storage containers. |
| Earlier: boundary classification | **RESOLVED** | Line 61 enumerates the five assessments: caller input, OS stdout, unauthenticated local peer, privacy transformation, Actions→contributor provenance. It separates validation/privacy/provenance boundaries from proven trust-zone changes [S17]. Checksum ≠ signer authenticity is noted. |
| Earlier: raw `.evidence/` store | **CORRECTED in this note** | §3 now says `.evidence/` is a README/gitignore convention only (`README.md:68`, `.gitignore:2`). Snapshot paths are caller-chosen and the diff goes to stdout. Plan line 59 agrees: no automatic raw persistence. |
| Earlier: no "experimental" claim | **CONFIRMED** | Neither this note nor the plan (lines 108, 133) claims Mermaid C4 is experimental. |

### Second recheck (t9 parent corrections)

**Rechecked file:** the same plan, 154 lines.

- SHA-256: `ab93a8fcd161437aeb18db56ed26157700052266cb3e003ab71dbae8a6bed63b`.
- Read on 2026-11-12, after the lead reported this as the final edit for this correction.

| Item | Disposition | Evidence |
| --- | --- | --- |
| One-binary/one-container scoped to this model | **RESOLVED** | Line 7 calls it a scoped modelling decision for this SwitchBridge model, "not a universal C4 rule". Line 105 says binary reuse or packaging alone do not determine container boundaries. Both are consistent with §2–§3 of this note, which were revised the same way. |
| DFD flow origins | **RESOLVED** | Line 107 allows process→process and process→store flows and keeps the no direct store→store rule. This matches [S13][S14][S16] and the revised §1.4 of this note. |
| Line 105 "C4 also permits filesystem containers" | **INFO** | [S3] verifies that containers include data stores. My fetch did not capture an explicit file-system example, so this review neither confirms nor disputes that specific wording. The lead reports independently re-opening [S3]. Non-blocking. |

**Open material findings: none.** Remaining review gates are semantic review of the actual authored views and visual review of rendered artifacts at pinned hashes (plan Tasks 3–4).

**Not reviewed:** the observer script, sibling LEA plans, and live PR/CI state. These require network or state checks outside my assignment.

## Sources

All accessed 2026-11-12.

- **S1** C4 model — Abstractions: <https://c4model.com/abstractions>
- **S2** C4 model — Software system: <https://c4model.com/abstractions/software-system>
- **S3** C4 model — Container: <https://c4model.com/abstractions/container>
- **S4** C4 model — Component: <https://c4model.com/abstractions/component>
- **S5** C4 model — System context diagram: <https://c4model.com/diagrams/system-context>
- **S6** C4 model — Container diagram: <https://c4model.com/diagrams/container>
- **S7** C4 model — Dynamic diagram: <https://c4model.com/diagrams/dynamic>
- **S8** C4 model — Deployment diagram: <https://c4model.com/diagrams/deployment>
- **S9** C4 model — Notation: <https://c4model.com/diagrams/notation>
- **S10** C4 model — Review checklist: <https://c4model.com/diagrams/checklist>
- **S11** C4 model — FAQ: <https://c4model.com/faq>
- **S12** Microsoft Learn — Create a threat model using data-flow diagram elements (module index): <https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/>
- **S13** …/2-process-the-task-element: <https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/2-process-the-task-element>
- **S14** …/3-data-store-the-storage-element: <https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/3-data-store-the-storage-element>
- **S15** …/4-external-entity-the-no-control-element: <https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/4-external-entity-the-no-control-element>
- **S16** …/5-data-flow-the-data-in-transit-element: <https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/5-data-flow-the-data-in-transit-element>
- **S17** …/6-trust-boundary-the-trust-zone-change-element: <https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/6-trust-boundary-the-trust-zone-change-element>
- **S18** Microsoft Learn — Microsoft Threat Modeling Tool feature overview: <https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-feature-overview>
- **S19** Mermaid — C4 diagrams (page showed v12.3.0): <https://mermaid.js.org/syntax/c4.html>
- **S20** Mermaid — Flowchart syntax: <https://mermaid.js.org/syntax/flowchart.html>
- **S21** Mermaid — Accessibility options: <https://mermaid.js.org/config/accessibility.html>
