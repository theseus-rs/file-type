use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113470579: FileFormat = FileFormat {
    id: 113_470_579,
    puid: "wikidata/113470579",
    name: "Microsoft Word for MS-DOS Printer Description File",
    extensions: &["prd"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
