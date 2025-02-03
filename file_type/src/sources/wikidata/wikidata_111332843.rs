use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111332843: FileFormat = FileFormat {
    id: 111_332_843,
    source_type: SourceType::Wikidata,
    name: "Roland S-7xx series floppy disk image",
    extensions: &["out", "sdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
