use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58960003: FileFormat = FileFormat {
    id: 58_960_003,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Chart, version 3",
    extensions: &["slc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
