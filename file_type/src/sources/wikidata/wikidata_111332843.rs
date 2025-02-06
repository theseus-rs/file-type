use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111332843: FileFormat = FileFormat {
    id: 111_332_843,
    source_type: SourceType::Wikidata,
    name: "Roland S-7xx series floppy disk image",
    extensions: &["out", "sdk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
