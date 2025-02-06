use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206229: FileFormat = FileFormat {
    id: 28_206_229,
    source_type: SourceType::Wikidata,
    name: "Gridded Binary",
    extensions: &["grb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
