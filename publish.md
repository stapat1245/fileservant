# Publishing a new PPA release

## 1. Update package version

```bash
dch -v 0.1.X-0ppa1
```

Update `Cargo.toml` if the application version changed.

---

## 2. Clean

```bash
cargo clean
rm -rf target
```

---

## 3. Build locally

```bash
cargo run
```

Verify everything works.

---

## 4. Go to parent directory

```bash
cd ..
```

---

## 5. Remove previous orig tarball

```bash
rm -f fileservant_0.1.X.orig.tar.xz
```

---

## 6. Create new orig tarball

```bash
tar \
    --exclude=.git \
    --exclude=target \
    -cJf fileservant_0.1.X.orig.tar.xz \
    fileservant-0.1.X
```

---

## 7. Build source package

```bash
cd fileservant-0.1.X

debuild -S -sa -us -uc
```

---

## 8. Sign package

```bash
cd ..

debsign \
-k4890CAE1CAF15A314A3CD9202619C395A8E3C254 \
fileservant_0.1.X-0ppa1_source.changes
```

---

## 9. Verify signatures

```bash
gpg --verify fileservant_0.1.X-0ppa1.dsc
gpg --verify fileservant_0.1.X-0ppa1_source.changes
```

Both should report:

```
Good signature
```

---

## 10. Upload

```bash
dput ppa:stapat/fileservant \
fileservant_0.1.X-0ppa1_source.changes
```

---

## 11. Wait for Launchpad

* Source upload accepted
* Package builds
* Binary published

---

## 12. Test

```bash
sudo apt update
apt policy fileservant
sudo apt install fileservant
```
