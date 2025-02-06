use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205569: FileFormat = FileFormat {
    id: 28_205_569,
    source_type: SourceType::Wikidata,
    name: "Nokia Startup Logo",
    extensions: &["nsl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
