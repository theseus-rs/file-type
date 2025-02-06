use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_86451671: FileFormat = FileFormat {
    id: 86_451_671,
    source_type: SourceType::Wikidata,
    name: "RFFlow Chart 4",
    extensions: &["flo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
