use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111395832: FileFormat = FileFormat {
    id: 111_395_832,
    source_type: SourceType::Wikidata,
    name: "PhotoSuite Slide Show File",
    extensions: &["pzs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
