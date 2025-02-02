use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63061396: FileFormat = FileFormat {
    id: 63_061_396,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word Document",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
