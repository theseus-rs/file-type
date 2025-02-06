use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272306: FileFormat = FileFormat {
    id: 111_272_306,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS instrument file",
    extensions: &["efe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
