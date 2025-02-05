use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129571777: FileFormat = FileFormat {
    id: 129_571_777,
    source_type: SourceType::Wikidata,
    name: "Hybris source code file",
    extensions: &["hyb"],
    media_types: &["application/x-hybris", "text/x-hybris"],
    signatures: &[],
    related_formats: &[],
};
