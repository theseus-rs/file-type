use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63061396: FileFormat = FileFormat {
    id: 63_061_396,
    puid: "wikidata/63061396",
    name: "Microsoft Word Document",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
