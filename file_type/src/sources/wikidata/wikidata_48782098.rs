use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48782098: FileFormat = FileFormat {
    id: 48_782_098,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for MS-DOS Document, version 3",
    extensions: &["doc"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
