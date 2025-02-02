use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129571777: FileFormat = FileFormat {
    id: 129_571_777,
    source_type: SourceType::Wikidata,
    name: "Hybris source code file",
    extensions: &["hyb"],
    media_types: &["application/x-hybris", "text/x-hybris"],
    internal_signatures: &[],
    related_formats: &[],
};
