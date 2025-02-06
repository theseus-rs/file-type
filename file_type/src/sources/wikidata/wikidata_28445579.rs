use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445579: FileFormat = FileFormat {
    id: 28_445_579,
    source_type: SourceType::Wikidata,
    name: "AcqKnowledge File",
    extensions: &["acq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
