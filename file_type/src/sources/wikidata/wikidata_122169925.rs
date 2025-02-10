use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122169925: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_925,
        source_type: SourceType::Wikidata,
        name: "GPU PWDUMP Password Hashes (NTLM)",
        extensions: &["pwdump"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
