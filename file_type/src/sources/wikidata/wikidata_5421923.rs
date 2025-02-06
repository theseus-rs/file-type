use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5421923: FileFormat = FileFormat {
    id: 5_421_923,
    source_type: SourceType::Wikidata,
    name: "Extensible MPEG-4 Textual Format",
    extensions: &["xmt"],
    media_types: &["application/mpeg4-iod-xmt"],
    signatures: &[],
    related_formats: &[],
};
