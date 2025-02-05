use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206310: FileFormat = FileFormat {
    id: 28_206_310,
    source_type: SourceType::Wikidata,
    name: "Analyze HDR",
    extensions: &["hdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
