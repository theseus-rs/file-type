use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129571777: FileFormat = FileFormat {
    id: 129_571_777,
    puid: "wikidata/129571777",
    name: "Hybris source code file",
    extensions: &["hyb", "hyb"],
    media_types: &["application/x-hybris", "text/x-hybris"],
    internal_signatures: &[],
    related_formats: &[],
};
