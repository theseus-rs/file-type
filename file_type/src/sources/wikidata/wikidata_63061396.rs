use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63061396: FileFormat = FileFormat {
    id: 63_061_396,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word Document",
    extensions: &["doc"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
