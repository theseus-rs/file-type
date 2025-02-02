use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857104: FileFormat = FileFormat {
    id: 105_857_104,
    source_type: SourceType::Wikidata,
    name: "GNU Privacy Guard public keyring (generic)",
    extensions: &["gpg"],
    media_types: &["application/pgp-keys"],
    internal_signatures: &[],
    related_formats: &[],
};
