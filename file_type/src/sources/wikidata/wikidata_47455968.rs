use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47455968: FileFormat = FileFormat {
    id: 47_455_968,
    source_type: SourceType::Wikidata,
    name: "SafeGuard Encrypted Virtual Disk",
    extensions: &["hdr", "vol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
