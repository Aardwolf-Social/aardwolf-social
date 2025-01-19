# Aardwolf Social
**Powering connected social communities with open-source software**

[![Aardwolf-Social/build](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/aardwolf-build.yml/badge.svg)](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/aardwolf-build.yml)
[![Aardwolf-Social/test](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/aardwolf-test.yml/badge.svg)](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/aardwolf-test.yml)
![GitHub issues](https://img.shields.io/github/issues/Aardwolf-Social/aardwolf)
[![rust-clippy analyze](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/rust-clippy.yml)
[![Docker Image CI](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/docker.yml/badge.svg)](https://github.com/Aardwolf-Social/aardwolf/actions/workflows/docker.yml)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)

---

<p align="center">
  <img alt="Aardwolf Social: Powering connected social communities with open software" src="/doc/images/aardwolf-banner_solid-bg.png" />
</p>

## 🌐 About Aardwolf Social

**Aardwolf Social** is a free and open-source alternative to Facebook that prioritizes user privacy and decentralization. Unlike mainstream platforms that present a single advertising-driven experience, Aardwolf empowers individuals and communities to create unique, customizable spaces while maintaining the ability to interact across different instances.

- **User Privacy First**: No ads, no tracking—just a community-first social experience.
- **Decentralized & Customizable**: Each community can have its own Aardwolf server (instance) that fits its specific needs.
- **Open Source**: Built by the community for the community.

---

## 📂 Project Structure

| Folder/File                  | Description                                       |
|------------------------------|---------------------------------------------------|
| `.github/`                    | CI/CD Files related to GitHub                     |
| `aardwolf-actix/`             | The Actix backend Rust application code           |
| `aardwolf-models/`            | Web app models and database setup files           |
| `aardwolf-templates/`         | Legacy frontend files                             |
| `aardwolf-test-helpers/`      | Development functional test code                  |
| `aardwolf-types/`             | Additional web app components                     |
| `aardwolf-yew-frontend/`      | The Yew frontend application code                 |
| `config/`                     | Aardwolf Social app configuration files           |
| `doc/`                        | Documentation                                     |
| `docker/`                     | Docker files                                      |
| `po/`                         | Legacy directory for i18n translations            |
| `src/`                        | The source directory for the main app             |
| `tests/`                      | Code validation and coverage tests                |
| `build.rs`                    | Rust code that directs Cargo build                |
| `Cargo.lock`                  | Complete manifest of all Rust crates used         |
| `Cargo.toml`                  | Manifest of crates required to build Aardwolf     |
| `CODE_OF_CONDUCT.md`          | Our Code of Conduct rules                         |
| `db-init.sh`                  | Part of the setup/install scripts                 |
| `diesel.toml`                 | Tells Diesel where to find the SQL migrations     |
| `LICENSE`                     | The license we use for this software              |
| `README.md`                   | The file you are presently reading                |
| `ROADMAP.md`                  | Our development roadmap                           |
| `rust-toolchain.toml`         | Specifies the Rust version for the dev environment|
| `SECURITY.md`                 | Future info for security updates                  |
| `translations/`               | Translations directory (links to `aardwolf-templates`) |

---

## 🌍 Screenshot of Aardwolf Social

Check out a preview of the homepage design below! This static demo showcases our design approach and upcoming features.

<p align="center">
  <img alt="Aardwolf Social: Powering connected social communities with open software" src="/doc/images/homepage-demo.png" />
</p>

---

## 🤝 Contributing to Aardwolf Social

We would love your help! Whether you’re an experienced developer or just starting, there’s a place for you in the Aardwolf community. Here are some ways to contribute:

- **Rust Developers**: If you're proficient in or learning Rust, we need your expertise to improve our backend.
- **Frontend Developers**: Help us design a beautiful and functional user interface using HTML, CSS, and Yew.
- **Documentation**: Proofread, organize, and update our documentation.
- **Docker & VMs**: Assist in building Docker images for development environments.

### How to Contribute

Follow these steps to start contributing to Aardwolf Social:

1. **Fork the Repository**:
   - Navigate to the main repository [Aardwolf Social GitHub Repo](https://github.com/Aardwolf-Social/aardwolf).
   - Click the **Fork** button in the upper right corner to create a copy of the repository under your GitHub account.

2. **Clone the Repository**:
   - Open your terminal and clone the forked repository to your local machine:
     ```bash
     git clone https://github.com/YOUR-USERNAME/aardwolf.git
     cd aardwolf
     ```

3. **Create a New Branch**:
   - Before making any changes, create a new branch for your feature or fix. This keeps your contributions organized and separate from the main codebase:
     ```bash
     git checkout -b feature-name
     ```

4. **Make Your Changes**:
   - Add your contributions, whether it’s code, documentation, or other improvements. Once you're happy with your changes, stage and commit them:
     ```bash
     git add .
     git commit -m "Description of your changes"
     ```

5. **Push the Changes to Your Fork**:
   - Push the changes from your local machine to your forked repository:
     ```bash
     git push origin feature-name
     ```

6. **Create a Pull Request (PR)**:
   - Go to your forked repository on GitHub, and you should see a prompt to **Compare & pull request**.
   - Click that, and make sure you're merging into the **main** branch of the original `Aardwolf-Social/aardwolf` repository.
   - Provide a clear title and description for your pull request so that maintainers can easily understand your changes.

7. **Wait for Feedback**:
   - A project maintainer will review your PR. They may ask for changes or approve it. Feel free to discuss or clarify anything in the PR comments.

### Additional Contribution Resources

- Check out our [contributor guidelines](/CONTRIBUTING.md) for detailed rules and tips.
- View our [development roadmap](/ROADMAP.md) to see current priorities and future plans.

---

## 📂 Repositories

Aardwolf Social is divided into several repositories:

- **[Aardwolf-Social Main](https://github.com/Aardwolf-Social/aardwolf)**: The main project repository.
- **[Aardwolf Social Interface](https://github.com/Aardwolf-Social/aardwolf-interface)**: The frontend development repository.
- **[Aardwolf Social Website](https://github.com/Aardwolf-Social/aardwolf-website)**: Repository for the Jekyll-powered website.

---

## 📬 Contact Us

Have questions or want to join the conversation? We’re available on several platforms:

- **Matrix Chat Room**: [#aardwolf-discussion:matrix.org](https://matrix.to/#/#aardwolf-discussion:matrix.org)
- **Mastodon**: [@banjofox2@hackers.town](https://hackers.town/@banjofox2)

---

## 🌟 Community Code of Conduct
Aardwolf Social is dedicated to fostering a welcoming and supportive community. We expect all contributors to follow our [Code of Conduct](/CODE_OF_CONDUCT.md) both online and offline. Let's build something amazing together!

---

## 📜 License
All Aardwolf Social software is licensed under the [GNU Affero General Public License (AGPL v3)](http://www.gnu.org/licenses/agpl-3.0).

---

**Join Us in Building a More Open and Connected World with Aardwolf Social!**
