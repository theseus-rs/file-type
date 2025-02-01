use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51093445: FileFormat = FileFormat {
    id: 51_093_445,
    puid: "wikidata/51093445",
    name: "Microsoft Outlook Address Book",
    extensions: &["olk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
