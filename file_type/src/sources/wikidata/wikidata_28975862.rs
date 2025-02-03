use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975862: FileFormat = FileFormat {
    id: 28_975_862,
    source_type: SourceType::Wikidata,
    name: "OOGL Bezier Surface BBP",
    extensions: &["bbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
