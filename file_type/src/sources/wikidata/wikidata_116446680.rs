use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116446680: FileFormat = FileFormat {
    id: 116_446_680,
    puid: "wikidata/116446680",
    name: "Microsoft Works Spreadsheet",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
