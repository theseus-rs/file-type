use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48782202: FileFormat = FileFormat {
    id: 48_782_202,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for MS-DOS Document, version 4",
    extensions: &["doc"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
