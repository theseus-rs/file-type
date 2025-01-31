use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_731135: FileFormat = FileFormat {
    id: 731_135,
    puid: "wikidata/731135",
    name: "Microsoft Management Console",
    extensions: &["msc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
