use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113470579: FileFormat = FileFormat {
    id: 113_470_579,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for MS-DOS Printer Description File",
    extensions: &["prd"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
