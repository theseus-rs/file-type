use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857104: FileFormat = FileFormat {
    id: 105_857_104,
    source_type: SourceType::Wikidata,
    name: "GNU Privacy Guard public keyring (generic)",
    extensions: &["gpg"],
    media_types: &["application/pgp-keys"],
    signatures: &[],
    related_formats: &[],
};
