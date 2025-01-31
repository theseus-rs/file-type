use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440765: FileFormat = FileFormat {
    id: 111_440_765,
    puid: "wikidata/111440765",
    name: "Ruby source code",
    extensions: &["rb", "rb"],
    media_types: &["application/x-ruby", "text/x-ruby"],
    internal_signatures: &[],
    related_formats: &[],
};
