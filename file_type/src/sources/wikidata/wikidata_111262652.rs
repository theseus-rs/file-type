use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111262652: FileFormat = FileFormat {
    id: 111_262_652,
    source_type: SourceType::Wikidata,
    name: "Muon DS404 patch file",
    extensions: &["404"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
