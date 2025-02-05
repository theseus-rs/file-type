use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111395832: FileFormat = FileFormat {
    id: 111_395_832,
    source_type: SourceType::Wikidata,
    name: "PhotoSuite Slide Show File",
    extensions: &["pzs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
