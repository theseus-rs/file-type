use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129188124: FileFormat = FileFormat {
    id: 129_188_124,
    source_type: SourceType::Wikidata,
    name: "FreeFem++ source code file",
    extensions: &["edp"],
    media_types: &["text/x-freefem"],
    signatures: &[],
    related_formats: &[],
};
