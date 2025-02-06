use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127703853: FileFormat = FileFormat {
    id: 127_703_853,
    source_type: SourceType::Wikidata,
    name: "Idris source code file",
    extensions: &["idr"],
    media_types: &["text/x-idris"],
    signatures: &[],
    related_formats: &[],
};
