use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127805343: FileFormat = FileFormat {
    id: 127_805_343,
    source_type: SourceType::Wikidata,
    name: "njs script file",
    extensions: &["njs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
