use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::de::{Deserialize, Deserializer, Error as SerdeError};

use crate::{errors::*, types::*};
use serde::{de, Serialize};

#[allow(dead_code)]
pub fn from_json<'a, T>(json: &'a str) -> RTDResult<T>
where
    T: serde::de::Deserialize<'a>,
{
    Ok(serde_json::from_str(json)?)
}

/// All tdlib type abstract class defined the same behavior
pub trait RObject: Debug {
    #[doc(hidden)]
    fn extra(&self) -> Option<String>;
    fn client_id(&self) -> Option<i32>;
}

pub trait RFunction: Debug + RObject + Serialize {
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<'a, RObj: RObject> RObject for &'a RObj {
    fn extra(&self) -> Option<String> {
        (*self).extra()
    }
    fn client_id(&self) -> Option<i32> {
        (*self).client_id()
    }
}

impl<'a, RObj: RObject> RObject for &'a mut RObj {
    fn extra(&self) -> Option<String> {
        (**self).extra()
    }
    fn client_id(&self) -> Option<i32> {
        (**self).client_id()
    }
}

impl<'a, Fnc: RFunction> RFunction for &'a Fnc {}
impl<'a, Fnc: RFunction> RFunction for &'a mut Fnc {}

impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType
    for &'a AUTHENTICATIONCODETYPE
{
}
impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType
    for &'a mut AUTHENTICATIONCODETYPE
{
}

impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &'a AUTHORIZATIONSTATE {}
impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState
    for &'a mut AUTHORIZATIONSTATE
{
}

impl<'a, BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &'a BACKGROUNDFILL {}
impl<'a, BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &'a mut BACKGROUNDFILL {}

impl<'a, BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &'a BACKGROUNDTYPE {}
impl<'a, BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &'a mut BACKGROUNDTYPE {}

impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a CALLDISCARDREASON {}
impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a mut CALLDISCARDREASON {}

impl<'a, CALLPROBLEM: TDCallProblem> TDCallProblem for &'a CALLPROBLEM {}
impl<'a, CALLPROBLEM: TDCallProblem> TDCallProblem for &'a mut CALLPROBLEM {}

impl<'a, CALLSERVERTYPE: TDCallServerType> TDCallServerType for &'a CALLSERVERTYPE {}
impl<'a, CALLSERVERTYPE: TDCallServerType> TDCallServerType for &'a mut CALLSERVERTYPE {}

impl<'a, CALLSTATE: TDCallState> TDCallState for &'a CALLSTATE {}
impl<'a, CALLSTATE: TDCallState> TDCallState for &'a mut CALLSTATE {}

impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload
    for &'a CALLBACKQUERYPAYLOAD
{
}
impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload
    for &'a mut CALLBACKQUERYPAYLOAD
{
}

impl<'a, CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult
    for &'a CANTRANSFEROWNERSHIPRESULT
{
}
impl<'a, CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult
    for &'a mut CANTRANSFEROWNERSHIPRESULT
{
}

impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a CHATACTION {}
impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a mut CHATACTION {}

impl<'a, CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &'a CHATACTIONBAR {}
impl<'a, CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &'a mut CHATACTIONBAR {}

impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a CHATEVENTACTION {}
impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a mut CHATEVENTACTION {}

impl<'a, CHATLIST: TDChatList> TDChatList for &'a CHATLIST {}
impl<'a, CHATLIST: TDChatList> TDChatList for &'a mut CHATLIST {}

impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a CHATMEMBERSTATUS {}
impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a mut CHATMEMBERSTATUS {}

impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a CHATMEMBERSFILTER {}
impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a mut CHATMEMBERSFILTER {}

impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a CHATREPORTREASON {}
impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a mut CHATREPORTREASON {}

impl<'a, CHATSOURCE: TDChatSource> TDChatSource for &'a CHATSOURCE {}
impl<'a, CHATSOURCE: TDChatSource> TDChatSource for &'a mut CHATSOURCE {}

impl<'a, CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &'a CHATSTATISTICS {}
impl<'a, CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &'a mut CHATSTATISTICS {}

impl<'a, CHATTYPE: TDChatType> TDChatType for &'a CHATTYPE {}
impl<'a, CHATTYPE: TDChatType> TDChatType for &'a mut CHATTYPE {}

impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult
    for &'a CHECKCHATUSERNAMERESULT
{
}
impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult
    for &'a mut CHECKCHATUSERNAMERESULT
{
}

impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a CONNECTIONSTATE {}
impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a mut CONNECTIONSTATE {}

impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a DEVICETOKEN {}
impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a mut DEVICETOKEN {}

impl<'a, DICESTICKERS: TDDiceStickers> TDDiceStickers for &'a DICESTICKERS {}
impl<'a, DICESTICKERS: TDDiceStickers> TDDiceStickers for &'a mut DICESTICKERS {}

impl<'a, FILETYPE: TDFileType> TDFileType for &'a FILETYPE {}
impl<'a, FILETYPE: TDFileType> TDFileType for &'a mut FILETYPE {}

impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType
    for &'a INLINEKEYBOARDBUTTONTYPE
{
}
impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType
    for &'a mut INLINEKEYBOARDBUTTONTYPE
{
}

impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a INLINEQUERYRESULT {}
impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a mut INLINEQUERYRESULT {}

impl<'a, INPUTBACKGROUND: TDInputBackground> TDInputBackground for &'a INPUTBACKGROUND {}
impl<'a, INPUTBACKGROUND: TDInputBackground> TDInputBackground for &'a mut INPUTBACKGROUND {}

impl<'a, INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &'a INPUTCHATPHOTO {}
impl<'a, INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &'a mut INPUTCHATPHOTO {}

impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a INPUTCREDENTIALS {}
impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a mut INPUTCREDENTIALS {}

impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a INPUTFILE {}
impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a mut INPUTFILE {}

impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
    for &'a INPUTINLINEQUERYRESULT
{
}
impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
    for &'a mut INPUTINLINEQUERYRESULT
{
}

impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent
    for &'a INPUTMESSAGECONTENT
{
}
impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent
    for &'a mut INPUTMESSAGECONTENT
{
}

impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement
    for &'a INPUTPASSPORTELEMENT
{
}
impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement
    for &'a mut INPUTPASSPORTELEMENT
{
}

impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource>
    TDInputPassportElementErrorSource for &'a INPUTPASSPORTELEMENTERRORSOURCE
{
}
impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource>
    TDInputPassportElementErrorSource for &'a mut INPUTPASSPORTELEMENTERRORSOURCE
{
}

impl<'a, INPUTSTICKER: TDInputSticker> TDInputSticker for &'a INPUTSTICKER {}
impl<'a, INPUTSTICKER: TDInputSticker> TDInputSticker for &'a mut INPUTSTICKER {}

impl<'a, JSONVALUE: TDJsonValue> TDJsonValue for &'a JSONVALUE {}
impl<'a, JSONVALUE: TDJsonValue> TDJsonValue for &'a mut JSONVALUE {}

impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &'a KEYBOARDBUTTONTYPE {}
impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType
    for &'a mut KEYBOARDBUTTONTYPE
{
}

impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue
    for &'a LANGUAGEPACKSTRINGVALUE
{
}
impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue
    for &'a mut LANGUAGEPACKSTRINGVALUE
{
}

impl<'a, LOGSTREAM: TDLogStream> TDLogStream for &'a LOGSTREAM {}
impl<'a, LOGSTREAM: TDLogStream> TDLogStream for &'a mut LOGSTREAM {}

impl<'a, LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &'a LOGINURLINFO {}
impl<'a, LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &'a mut LOGINURLINFO {}

impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a MASKPOINT {}
impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a mut MASKPOINT {}

impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a MESSAGECONTENT {}
impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a mut MESSAGECONTENT {}

impl<'a, MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin
    for &'a MESSAGEFORWARDORIGIN
{
}
impl<'a, MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin
    for &'a mut MESSAGEFORWARDORIGIN
{
}

impl<'a, MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
    for &'a MESSAGESCHEDULINGSTATE
{
}
impl<'a, MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
    for &'a mut MESSAGESCHEDULINGSTATE
{
}

impl<'a, MESSAGESENDER: TDMessageSender> TDMessageSender for &'a MESSAGESENDER {}
impl<'a, MESSAGESENDER: TDMessageSender> TDMessageSender for &'a mut MESSAGESENDER {}

impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState
    for &'a MESSAGESENDINGSTATE
{
}
impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState
    for &'a mut MESSAGESENDINGSTATE
{
}

impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry
    for &'a NETWORKSTATISTICSENTRY
{
}
impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry
    for &'a mut NETWORKSTATISTICSENTRY
{
}

impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a NETWORKTYPE {}
impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a mut NETWORKTYPE {}

impl<'a, NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType
    for &'a NOTIFICATIONGROUPTYPE
{
}
impl<'a, NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType
    for &'a mut NOTIFICATIONGROUPTYPE
{
}

impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope
    for &'a NOTIFICATIONSETTINGSSCOPE
{
}
impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope
    for &'a mut NOTIFICATIONSETTINGSSCOPE
{
}

impl<'a, NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &'a NOTIFICATIONTYPE {}
impl<'a, NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &'a mut NOTIFICATIONTYPE {}

impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a OPTIONVALUE {}
impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a mut OPTIONVALUE {}

impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a PAGEBLOCK {}
impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a mut PAGEBLOCK {}

impl<'a, PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment>
    TDPageBlockHorizontalAlignment for &'a PAGEBLOCKHORIZONTALALIGNMENT
{
}
impl<'a, PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment>
    TDPageBlockHorizontalAlignment for &'a mut PAGEBLOCKHORIZONTALALIGNMENT
{
}

impl<'a, PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment
    for &'a PAGEBLOCKVERTICALALIGNMENT
{
}
impl<'a, PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment
    for &'a mut PAGEBLOCKVERTICALALIGNMENT
{
}

impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a PASSPORTELEMENT {}
impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a mut PASSPORTELEMENT {}

impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource
    for &'a PASSPORTELEMENTERRORSOURCE
{
}
impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource
    for &'a mut PASSPORTELEMENTERRORSOURCE
{
}

impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType
    for &'a PASSPORTELEMENTTYPE
{
}
impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType
    for &'a mut PASSPORTELEMENTTYPE
{
}

impl<'a, POLLTYPE: TDPollType> TDPollType for &'a POLLTYPE {}
impl<'a, POLLTYPE: TDPollType> TDPollType for &'a mut POLLTYPE {}

impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a PROXYTYPE {}
impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a mut PROXYTYPE {}

impl<'a, PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &'a PUBLICCHATTYPE {}
impl<'a, PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &'a mut PUBLICCHATTYPE {}

impl<'a, PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &'a PUSHMESSAGECONTENT {}
impl<'a, PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent
    for &'a mut PUSHMESSAGECONTENT
{
}

impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a REPLYMARKUP {}
impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a mut REPLYMARKUP {}

impl<'a, RICHTEXT: TDRichText> TDRichText for &'a RICHTEXT {}
impl<'a, RICHTEXT: TDRichText> TDRichText for &'a mut RICHTEXT {}

impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter
    for &'a SEARCHMESSAGESFILTER
{
}
impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter
    for &'a mut SEARCHMESSAGESFILTER
{
}

impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a SECRETCHATSTATE {}
impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a mut SECRETCHATSTATE {}

impl<'a, STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &'a STATISTICALGRAPH {}
impl<'a, STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &'a mut STATISTICALGRAPH {}

impl<'a, SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &'a SUGGESTEDACTION {}
impl<'a, SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &'a mut SUGGESTEDACTION {}

impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter
    for &'a SUPERGROUPMEMBERSFILTER
{
}
impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter
    for &'a mut SUPERGROUPMEMBERSFILTER
{
}

impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a TMEURLTYPE {}
impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a mut TMEURLTYPE {}

impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a TEXTENTITYTYPE {}
impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a mut TEXTENTITYTYPE {}

impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a TEXTPARSEMODE {}
impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a mut TEXTPARSEMODE {}

impl<'a, THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &'a THUMBNAILFORMAT {}
impl<'a, THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &'a mut THUMBNAILFORMAT {}

impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a TOPCHATCATEGORY {}
impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a mut TOPCHATCATEGORY {}

impl<'a, UPDATE: TDUpdate> TDUpdate for &'a UPDATE {}
impl<'a, UPDATE: TDUpdate> TDUpdate for &'a mut UPDATE {}

impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &'a USERPRIVACYSETTING {}
impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting
    for &'a mut USERPRIVACYSETTING
{
}

impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule
    for &'a USERPRIVACYSETTINGRULE
{
}
impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule
    for &'a mut USERPRIVACYSETTINGRULE
{
}

impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a USERSTATUS {}
impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a mut USERSTATUS {}

impl<'a, USERTYPE: TDUserType> TDUserType for &'a USERTYPE {}
impl<'a, USERTYPE: TDUserType> TDUserType for &'a mut USERTYPE {}

#[derive(Debug, Clone)]
pub enum TdType {
    AuthorizationState(AuthorizationState),
    CanTransferOwnershipResult(CanTransferOwnershipResult),
    ChatStatistics(ChatStatistics),
    CheckChatUsernameResult(CheckChatUsernameResult),
    JsonValue(JsonValue),
    LanguagePackStringValue(LanguagePackStringValue),
    LogStream(LogStream),
    LoginUrlInfo(LoginUrlInfo),
    OptionValue(OptionValue),
    PassportElement(PassportElement),
    StatisticalGraph(StatisticalGraph),
    Update(Update),
    AccountTtl(AccountTtl),
    Animations(Animations),
    AuthenticationCodeInfo(AuthenticationCodeInfo),
    AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
    Background(Background),
    Backgrounds(Backgrounds),
    BankCardInfo(BankCardInfo),
    BasicGroup(BasicGroup),
    BasicGroupFullInfo(BasicGroupFullInfo),
    CallId(CallId),
    CallbackQueryAnswer(CallbackQueryAnswer),
    Chat(Chat),
    ChatAdministrators(ChatAdministrators),
    ChatEvents(ChatEvents),
    ChatFilter(ChatFilter),
    ChatFilterInfo(ChatFilterInfo),
    ChatInviteLink(ChatInviteLink),
    ChatInviteLinkInfo(ChatInviteLinkInfo),
    ChatLists(ChatLists),
    ChatMember(ChatMember),
    ChatMembers(ChatMembers),
    ChatPhotos(ChatPhotos),
    Chats(Chats),
    ChatsNearby(ChatsNearby),
    ConnectedWebsites(ConnectedWebsites),
    Count(Count),
    Countries(Countries),
    CustomRequestResult(CustomRequestResult),
    DatabaseStatistics(DatabaseStatistics),
    DeepLinkInfo(DeepLinkInfo),
    EmailAddressAuthenticationCodeInfo(EmailAddressAuthenticationCodeInfo),
    Emojis(Emojis),
    Error(Error),
    File(File),
    FilePart(FilePart),
    FormattedText(FormattedText),
    FoundMessages(FoundMessages),
    GameHighScores(GameHighScores),
    Hashtags(Hashtags),
    HttpUrl(HttpUrl),
    ImportedContacts(ImportedContacts),
    InlineQueryResults(InlineQueryResults),
    LanguagePackInfo(LanguagePackInfo),
    LanguagePackStrings(LanguagePackStrings),
    LocalizationTargetInfo(LocalizationTargetInfo),
    LogTags(LogTags),
    LogVerbosityLevel(LogVerbosityLevel),
    Message(Message),
    MessageLink(MessageLink),
    MessageLinkInfo(MessageLinkInfo),
    MessageSenders(MessageSenders),
    MessageStatistics(MessageStatistics),
    MessageThreadInfo(MessageThreadInfo),
    Messages(Messages),
    NetworkStatistics(NetworkStatistics),
    Ok(Ok),
    OrderInfo(OrderInfo),
    PassportAuthorizationForm(PassportAuthorizationForm),
    PassportElements(PassportElements),
    PassportElementsWithErrors(PassportElementsWithErrors),
    PasswordState(PasswordState),
    PaymentForm(PaymentForm),
    PaymentReceipt(PaymentReceipt),
    PaymentResult(PaymentResult),
    PhoneNumberInfo(PhoneNumberInfo),
    Proxies(Proxies),
    Proxy(Proxy),
    PushReceiverId(PushReceiverId),
    RecommendedChatFilters(RecommendedChatFilters),
    RecoveryEmailAddress(RecoveryEmailAddress),
    ScopeNotificationSettings(ScopeNotificationSettings),
    Seconds(Seconds),
    SecretChat(SecretChat),
    Session(Session),
    Sessions(Sessions),
    StickerSet(StickerSet),
    StickerSets(StickerSets),
    Stickers(Stickers),
    StorageStatistics(StorageStatistics),
    StorageStatisticsFast(StorageStatisticsFast),
    Supergroup(Supergroup),
    SupergroupFullInfo(SupergroupFullInfo),
    TMeUrls(TMeUrls),
    TemporaryPasswordState(TemporaryPasswordState),
    TestBytes(TestBytes),
    TestInt(TestInt),
    TestString(TestString),
    TestVectorInt(TestVectorInt),
    TestVectorIntObject(TestVectorIntObject),
    TestVectorString(TestVectorString),
    TestVectorStringObject(TestVectorStringObject),
    Text(Text),
    TextEntities(TextEntities),
    Updates(Updates),
    User(User),
    UserFullInfo(UserFullInfo),
    UserPrivacySettingRules(UserPrivacySettingRules),
    Users(Users),
    ValidatedOrderInfo(ValidatedOrderInfo),
    WebPage(WebPage),
    WebPageInstantView(WebPageInstantView),
}
impl<'de> Deserialize<'de> for TdType {
    fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let rtd_trait_map = match rtd_trait_value.as_object() {
            Some(map) => map,
            None => {
                return Err(D::Error::unknown_field(
                    stringify!(TdType),
                    &[stringify!("{} is not the correct type", TdType)],
                ))
            }
        };

        let rtd_trait_type = match rtd_trait_map.get("@type") {
            Some(t) => match t.as_str() {
                Some(s) => s,
                None => {
                    return Err(D::Error::unknown_field(
                        stringify!( "{} -> @type" , $field ),
                        &[stringify!("{} -> @type is not the correct type", TdType)],
                    ))
                }
            },
            None => return Err(D::Error::custom("@type is empty")),
        };
        if let Some(t) =
            deserialize_traits(rtd_trait_type, rtd_trait_value.clone()).map_err(|err| {
                D::Error::custom(format!(
                    "can't deserialize for {} with error: {}",
                    rtd_trait_type, err
                ))
            })?
        {
            return Ok(t);
        };
        if let Some(t) =
            deserialize_direct_types(rtd_trait_type, rtd_trait_value.clone()).map_err(|err| {
                D::Error::custom(format!(
                    "can't deserialize for {} with error: {}",
                    rtd_trait_type, err
                ))
            })?
        {
            return Ok(t);
        }
        return Err(D::Error::custom(format!(
            "got {} @type with unavailable variant",
            rtd_trait_type
        )));
    }
}

fn deserialize_traits(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    if let Some(td_type) = deserialize_authorization_state(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_can_transfer_ownership_result(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_chat_statistics(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_check_chat_username_result(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_json_value(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_language_pack_string_value(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_log_stream(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_login_url_info(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_option_value(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_passport_element(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_statistical_graph(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_update(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    Ok(None)
}

fn deserialize_direct_types(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match rtd_trait_type {
        "accountTtl" => Some(TdType::AccountTtl(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "animations" => Some(TdType::Animations(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "authenticationCodeInfo" => Some(TdType::AuthenticationCodeInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "autoDownloadSettingsPresets" => Some(TdType::AutoDownloadSettingsPresets(
            serde_json::from_value(rtd_trait_value.clone())?,
        )),
        "background" => Some(TdType::Background(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "backgrounds" => Some(TdType::Backgrounds(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "bankCardInfo" => Some(TdType::BankCardInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "basicGroup" => Some(TdType::BasicGroup(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "basicGroupFullInfo" => Some(TdType::BasicGroupFullInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "callId" => Some(TdType::CallId(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "callbackQueryAnswer" => Some(TdType::CallbackQueryAnswer(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chat" => Some(TdType::Chat(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatAdministrators" => Some(TdType::ChatAdministrators(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatEvents" => Some(TdType::ChatEvents(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatFilter" => Some(TdType::ChatFilter(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatFilterInfo" => Some(TdType::ChatFilterInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatInviteLink" => Some(TdType::ChatInviteLink(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatInviteLinkInfo" => Some(TdType::ChatInviteLinkInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatLists" => Some(TdType::ChatLists(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatMember" => Some(TdType::ChatMember(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatMembers" => Some(TdType::ChatMembers(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatPhotos" => Some(TdType::ChatPhotos(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chats" => Some(TdType::Chats(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "chatsNearby" => Some(TdType::ChatsNearby(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "connectedWebsites" => Some(TdType::ConnectedWebsites(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "count" => Some(TdType::Count(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "countries" => Some(TdType::Countries(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "customRequestResult" => Some(TdType::CustomRequestResult(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "databaseStatistics" => Some(TdType::DatabaseStatistics(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "deepLinkInfo" => Some(TdType::DeepLinkInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "emailAddressAuthenticationCodeInfo" => Some(TdType::EmailAddressAuthenticationCodeInfo(
            serde_json::from_value(rtd_trait_value.clone())?,
        )),
        "emojis" => Some(TdType::Emojis(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "error" => Some(TdType::Error(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "file" => Some(TdType::File(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "filePart" => Some(TdType::FilePart(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "formattedText" => Some(TdType::FormattedText(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "foundMessages" => Some(TdType::FoundMessages(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "gameHighScores" => Some(TdType::GameHighScores(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "hashtags" => Some(TdType::Hashtags(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "httpUrl" => Some(TdType::HttpUrl(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "importedContacts" => Some(TdType::ImportedContacts(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "inlineQueryResults" => Some(TdType::InlineQueryResults(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "languagePackInfo" => Some(TdType::LanguagePackInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "languagePackStrings" => Some(TdType::LanguagePackStrings(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "localizationTargetInfo" => Some(TdType::LocalizationTargetInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "logTags" => Some(TdType::LogTags(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "logVerbosityLevel" => Some(TdType::LogVerbosityLevel(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "message" => Some(TdType::Message(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "messageLink" => Some(TdType::MessageLink(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "messageLinkInfo" => Some(TdType::MessageLinkInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "messageSenders" => Some(TdType::MessageSenders(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "messageStatistics" => Some(TdType::MessageStatistics(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "messageThreadInfo" => Some(TdType::MessageThreadInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "messages" => Some(TdType::Messages(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "networkStatistics" => Some(TdType::NetworkStatistics(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "ok" => Some(TdType::Ok(serde_json::from_value(rtd_trait_value.clone())?)),
        "orderInfo" => Some(TdType::OrderInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "passportAuthorizationForm" => Some(TdType::PassportAuthorizationForm(
            serde_json::from_value(rtd_trait_value.clone())?,
        )),
        "passportElements" => Some(TdType::PassportElements(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "passportElementsWithErrors" => Some(TdType::PassportElementsWithErrors(
            serde_json::from_value(rtd_trait_value.clone())?,
        )),
        "passwordState" => Some(TdType::PasswordState(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "paymentForm" => Some(TdType::PaymentForm(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "paymentReceipt" => Some(TdType::PaymentReceipt(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "paymentResult" => Some(TdType::PaymentResult(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "phoneNumberInfo" => Some(TdType::PhoneNumberInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "proxies" => Some(TdType::Proxies(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "proxy" => Some(TdType::Proxy(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "pushReceiverId" => Some(TdType::PushReceiverId(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "recommendedChatFilters" => Some(TdType::RecommendedChatFilters(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "recoveryEmailAddress" => Some(TdType::RecoveryEmailAddress(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "scopeNotificationSettings" => Some(TdType::ScopeNotificationSettings(
            serde_json::from_value(rtd_trait_value.clone())?,
        )),
        "seconds" => Some(TdType::Seconds(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "secretChat" => Some(TdType::SecretChat(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "session" => Some(TdType::Session(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "sessions" => Some(TdType::Sessions(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "stickerSet" => Some(TdType::StickerSet(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "stickerSets" => Some(TdType::StickerSets(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "stickers" => Some(TdType::Stickers(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "storageStatistics" => Some(TdType::StorageStatistics(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "storageStatisticsFast" => Some(TdType::StorageStatisticsFast(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "supergroup" => Some(TdType::Supergroup(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "supergroupFullInfo" => Some(TdType::SupergroupFullInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "tMeUrls" => Some(TdType::TMeUrls(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "temporaryPasswordState" => Some(TdType::TemporaryPasswordState(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testBytes" => Some(TdType::TestBytes(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testInt" => Some(TdType::TestInt(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testString" => Some(TdType::TestString(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testVectorInt" => Some(TdType::TestVectorInt(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testVectorIntObject" => Some(TdType::TestVectorIntObject(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testVectorString" => Some(TdType::TestVectorString(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "testVectorStringObject" => Some(TdType::TestVectorStringObject(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "text" => Some(TdType::Text(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "textEntities" => Some(TdType::TextEntities(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "updates" => Some(TdType::Updates(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "user" => Some(TdType::User(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "userFullInfo" => Some(TdType::UserFullInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "userPrivacySettingRules" => Some(TdType::UserPrivacySettingRules(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "users" => Some(TdType::Users(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "validatedOrderInfo" => Some(TdType::ValidatedOrderInfo(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "webPage" => Some(TdType::WebPage(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        "webPageInstantView" => Some(TdType::WebPageInstantView(serde_json::from_value(
            rtd_trait_value.clone(),
        )?)),
        _ => None,
    })
}

const AuthorizationState_MEMBERS: &'static [&'static str] = &[
    "authorizationStateClosed",
    "authorizationStateClosing",
    "authorizationStateLoggingOut",
    "authorizationStateReady",
    "authorizationStateWaitCode",
    "authorizationStateWaitEncryptionKey",
    "authorizationStateWaitOtherDeviceConfirmation",
    "authorizationStateWaitPassword",
    "authorizationStateWaitPhoneNumber",
    "authorizationStateWaitRegistration",
    "authorizationStateWaitTdlibParameters",
    "getAuthorizationState",
];

fn deserialize_authorization_state(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match AuthorizationState_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::AuthorizationState(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const CanTransferOwnershipResult_MEMBERS: &'static [&'static str] = &[
    "canTransferOwnership",
    "canTransferOwnershipResultOk",
    "canTransferOwnershipResultPasswordNeeded",
    "canTransferOwnershipResultPasswordTooFresh",
    "canTransferOwnershipResultSessionTooFresh",
];

fn deserialize_can_transfer_ownership_result(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match CanTransferOwnershipResult_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::CanTransferOwnershipResult(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const ChatStatistics_MEMBERS: &'static [&'static str] = &[
    "chatStatisticsChannel",
    "chatStatisticsSupergroup",
    "getChatStatistics",
];

fn deserialize_chat_statistics(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match ChatStatistics_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::ChatStatistics(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const CheckChatUsernameResult_MEMBERS: &'static [&'static str] = &[
    "checkChatUsername",
    "checkChatUsernameResultOk",
    "checkChatUsernameResultPublicChatsTooMuch",
    "checkChatUsernameResultPublicGroupsUnavailable",
    "checkChatUsernameResultUsernameInvalid",
    "checkChatUsernameResultUsernameOccupied",
];

fn deserialize_check_chat_username_result(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match CheckChatUsernameResult_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::CheckChatUsernameResult(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const JsonValue_MEMBERS: &'static [&'static str] = &[
    "getApplicationConfig",
    "getJsonValue",
    "jsonValueArray",
    "jsonValueBoolean",
    "jsonValueNull",
    "jsonValueNumber",
    "jsonValueObject",
    "jsonValueString",
];

fn deserialize_json_value(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match JsonValue_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::JsonValue(serde_json::from_value(rtd_trait_value)?)),
        false => None,
    })
}

const LanguagePackStringValue_MEMBERS: &'static [&'static str] = &[
    "getLanguagePackString",
    "languagePackStringValueDeleted",
    "languagePackStringValueOrdinary",
    "languagePackStringValuePluralized",
];

fn deserialize_language_pack_string_value(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match LanguagePackStringValue_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::LanguagePackStringValue(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const LogStream_MEMBERS: &'static [&'static str] = &[
    "getLogStream",
    "logStreamDefault",
    "logStreamEmpty",
    "logStreamFile",
];

fn deserialize_log_stream(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match LogStream_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::LogStream(serde_json::from_value(rtd_trait_value)?)),
        false => None,
    })
}

const LoginUrlInfo_MEMBERS: &'static [&'static str] = &[
    "getLoginUrlInfo",
    "loginUrlInfoOpen",
    "loginUrlInfoRequestConfirmation",
];

fn deserialize_login_url_info(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match LoginUrlInfo_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::LoginUrlInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const OptionValue_MEMBERS: &'static [&'static str] = &[
    "getOption",
    "optionValueBoolean",
    "optionValueEmpty",
    "optionValueInteger",
    "optionValueString",
];

fn deserialize_option_value(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match OptionValue_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::OptionValue(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const PassportElement_MEMBERS: &'static [&'static str] = &[
    "getPassportElement",
    "passportElementAddress",
    "passportElementBankStatement",
    "passportElementDriverLicense",
    "passportElementEmailAddress",
    "passportElementIdentityCard",
    "passportElementInternalPassport",
    "passportElementPassport",
    "passportElementPassportRegistration",
    "passportElementPersonalDetails",
    "passportElementPhoneNumber",
    "passportElementRentalAgreement",
    "passportElementTemporaryRegistration",
    "passportElementUtilityBill",
    "setPassportElement",
];

fn deserialize_passport_element(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match PassportElement_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::PassportElement(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const StatisticalGraph_MEMBERS: &'static [&'static str] = &[
    "getStatisticalGraph",
    "statisticalGraphAsync",
    "statisticalGraphData",
    "statisticalGraphError",
];

fn deserialize_statistical_graph(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match StatisticalGraph_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::StatisticalGraph(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const Update_MEMBERS: &'static [&'static str] = &[
    "testUseUpdate",
    "updateActiveNotifications",
    "updateAnimationSearchParameters",
    "updateAuthorizationState",
    "updateBasicGroup",
    "updateBasicGroupFullInfo",
    "updateCall",
    "updateChatActionBar",
    "updateChatDefaultDisableNotification",
    "updateChatDraftMessage",
    "updateChatFilters",
    "updateChatHasScheduledMessages",
    "updateChatIsBlocked",
    "updateChatIsMarkedAsUnread",
    "updateChatLastMessage",
    "updateChatNotificationSettings",
    "updateChatOnlineMemberCount",
    "updateChatPermissions",
    "updateChatPhoto",
    "updateChatPosition",
    "updateChatReadInbox",
    "updateChatReadOutbox",
    "updateChatReplyMarkup",
    "updateChatTitle",
    "updateChatUnreadMentionCount",
    "updateConnectionState",
    "updateDeleteMessages",
    "updateDiceEmojis",
    "updateFavoriteStickers",
    "updateFile",
    "updateFileGenerationStart",
    "updateFileGenerationStop",
    "updateHavePendingNotifications",
    "updateInstalledStickerSets",
    "updateLanguagePackStrings",
    "updateMessageContent",
    "updateMessageContentOpened",
    "updateMessageEdited",
    "updateMessageInteractionInfo",
    "updateMessageIsPinned",
    "updateMessageLiveLocationViewed",
    "updateMessageMentionRead",
    "updateMessageSendAcknowledged",
    "updateMessageSendFailed",
    "updateMessageSendSucceeded",
    "updateNewCallSignalingData",
    "updateNewCallbackQuery",
    "updateNewChat",
    "updateNewChosenInlineResult",
    "updateNewCustomEvent",
    "updateNewCustomQuery",
    "updateNewInlineCallbackQuery",
    "updateNewInlineQuery",
    "updateNewMessage",
    "updateNewPreCheckoutQuery",
    "updateNewShippingQuery",
    "updateNotification",
    "updateNotificationGroup",
    "updateOption",
    "updatePoll",
    "updatePollAnswer",
    "updateRecentStickers",
    "updateSavedAnimations",
    "updateScopeNotificationSettings",
    "updateSecretChat",
    "updateSelectedBackground",
    "updateServiceNotification",
    "updateStickerSet",
    "updateSuggestedActions",
    "updateSupergroup",
    "updateSupergroupFullInfo",
    "updateTermsOfService",
    "updateTrendingStickerSets",
    "updateUnreadChatCount",
    "updateUnreadMessageCount",
    "updateUser",
    "updateUserChatAction",
    "updateUserFullInfo",
    "updateUserPrivacySettingRules",
    "updateUserStatus",
    "updateUsersNearby",
];

fn deserialize_update(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match Update_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::Update(serde_json::from_value(rtd_trait_value)?)),
        false => None,
    })
}

impl TdType {
    pub fn client_id(&self) -> Option<i32> {
        match self {
            TdType::AuthorizationState(value) => value.client_id(),

            TdType::CanTransferOwnershipResult(value) => value.client_id(),

            TdType::ChatStatistics(value) => value.client_id(),

            TdType::CheckChatUsernameResult(value) => value.client_id(),

            TdType::JsonValue(value) => value.client_id(),

            TdType::LanguagePackStringValue(value) => value.client_id(),

            TdType::LogStream(value) => value.client_id(),

            TdType::LoginUrlInfo(value) => value.client_id(),

            TdType::OptionValue(value) => value.client_id(),

            TdType::PassportElement(value) => value.client_id(),

            TdType::StatisticalGraph(value) => value.client_id(),

            TdType::Update(value) => value.client_id(),

            TdType::AccountTtl(value) => value.client_id(),

            TdType::Animations(value) => value.client_id(),

            TdType::AuthenticationCodeInfo(value) => value.client_id(),

            TdType::AutoDownloadSettingsPresets(value) => value.client_id(),

            TdType::Background(value) => value.client_id(),

            TdType::Backgrounds(value) => value.client_id(),

            TdType::BankCardInfo(value) => value.client_id(),

            TdType::BasicGroup(value) => value.client_id(),

            TdType::BasicGroupFullInfo(value) => value.client_id(),

            TdType::CallId(value) => value.client_id(),

            TdType::CallbackQueryAnswer(value) => value.client_id(),

            TdType::Chat(value) => value.client_id(),

            TdType::ChatAdministrators(value) => value.client_id(),

            TdType::ChatEvents(value) => value.client_id(),

            TdType::ChatFilter(value) => value.client_id(),

            TdType::ChatFilterInfo(value) => value.client_id(),

            TdType::ChatInviteLink(value) => value.client_id(),

            TdType::ChatInviteLinkInfo(value) => value.client_id(),

            TdType::ChatLists(value) => value.client_id(),

            TdType::ChatMember(value) => value.client_id(),

            TdType::ChatMembers(value) => value.client_id(),

            TdType::ChatPhotos(value) => value.client_id(),

            TdType::Chats(value) => value.client_id(),

            TdType::ChatsNearby(value) => value.client_id(),

            TdType::ConnectedWebsites(value) => value.client_id(),

            TdType::Count(value) => value.client_id(),

            TdType::Countries(value) => value.client_id(),

            TdType::CustomRequestResult(value) => value.client_id(),

            TdType::DatabaseStatistics(value) => value.client_id(),

            TdType::DeepLinkInfo(value) => value.client_id(),

            TdType::EmailAddressAuthenticationCodeInfo(value) => value.client_id(),

            TdType::Emojis(value) => value.client_id(),

            TdType::Error(value) => value.client_id(),

            TdType::File(value) => value.client_id(),

            TdType::FilePart(value) => value.client_id(),

            TdType::FormattedText(value) => value.client_id(),

            TdType::FoundMessages(value) => value.client_id(),

            TdType::GameHighScores(value) => value.client_id(),

            TdType::Hashtags(value) => value.client_id(),

            TdType::HttpUrl(value) => value.client_id(),

            TdType::ImportedContacts(value) => value.client_id(),

            TdType::InlineQueryResults(value) => value.client_id(),

            TdType::LanguagePackInfo(value) => value.client_id(),

            TdType::LanguagePackStrings(value) => value.client_id(),

            TdType::LocalizationTargetInfo(value) => value.client_id(),

            TdType::LogTags(value) => value.client_id(),

            TdType::LogVerbosityLevel(value) => value.client_id(),

            TdType::Message(value) => value.client_id(),

            TdType::MessageLink(value) => value.client_id(),

            TdType::MessageLinkInfo(value) => value.client_id(),

            TdType::MessageSenders(value) => value.client_id(),

            TdType::MessageStatistics(value) => value.client_id(),

            TdType::MessageThreadInfo(value) => value.client_id(),

            TdType::Messages(value) => value.client_id(),

            TdType::NetworkStatistics(value) => value.client_id(),

            TdType::Ok(value) => value.client_id(),

            TdType::OrderInfo(value) => value.client_id(),

            TdType::PassportAuthorizationForm(value) => value.client_id(),

            TdType::PassportElements(value) => value.client_id(),

            TdType::PassportElementsWithErrors(value) => value.client_id(),

            TdType::PasswordState(value) => value.client_id(),

            TdType::PaymentForm(value) => value.client_id(),

            TdType::PaymentReceipt(value) => value.client_id(),

            TdType::PaymentResult(value) => value.client_id(),

            TdType::PhoneNumberInfo(value) => value.client_id(),

            TdType::Proxies(value) => value.client_id(),

            TdType::Proxy(value) => value.client_id(),

            TdType::PushReceiverId(value) => value.client_id(),

            TdType::RecommendedChatFilters(value) => value.client_id(),

            TdType::RecoveryEmailAddress(value) => value.client_id(),

            TdType::ScopeNotificationSettings(value) => value.client_id(),

            TdType::Seconds(value) => value.client_id(),

            TdType::SecretChat(value) => value.client_id(),

            TdType::Session(value) => value.client_id(),

            TdType::Sessions(value) => value.client_id(),

            TdType::StickerSet(value) => value.client_id(),

            TdType::StickerSets(value) => value.client_id(),

            TdType::Stickers(value) => value.client_id(),

            TdType::StorageStatistics(value) => value.client_id(),

            TdType::StorageStatisticsFast(value) => value.client_id(),

            TdType::Supergroup(value) => value.client_id(),

            TdType::SupergroupFullInfo(value) => value.client_id(),

            TdType::TMeUrls(value) => value.client_id(),

            TdType::TemporaryPasswordState(value) => value.client_id(),

            TdType::TestBytes(value) => value.client_id(),

            TdType::TestInt(value) => value.client_id(),

            TdType::TestString(value) => value.client_id(),

            TdType::TestVectorInt(value) => value.client_id(),

            TdType::TestVectorIntObject(value) => value.client_id(),

            TdType::TestVectorString(value) => value.client_id(),

            TdType::TestVectorStringObject(value) => value.client_id(),

            TdType::Text(value) => value.client_id(),

            TdType::TextEntities(value) => value.client_id(),

            TdType::Updates(value) => value.client_id(),

            TdType::User(value) => value.client_id(),

            TdType::UserFullInfo(value) => value.client_id(),

            TdType::UserPrivacySettingRules(value) => value.client_id(),

            TdType::Users(value) => value.client_id(),

            TdType::ValidatedOrderInfo(value) => value.client_id(),

            TdType::WebPage(value) => value.client_id(),

            TdType::WebPageInstantView(value) => value.client_id(),
        }
    }

    pub fn extra(&self) -> Option<String> {
        match self {
            TdType::AuthorizationState(value) => value.extra(),

            TdType::CanTransferOwnershipResult(value) => value.extra(),

            TdType::ChatStatistics(value) => value.extra(),

            TdType::CheckChatUsernameResult(value) => value.extra(),

            TdType::JsonValue(value) => value.extra(),

            TdType::LanguagePackStringValue(value) => value.extra(),

            TdType::LogStream(value) => value.extra(),

            TdType::LoginUrlInfo(value) => value.extra(),

            TdType::OptionValue(value) => value.extra(),

            TdType::PassportElement(value) => value.extra(),

            TdType::StatisticalGraph(value) => value.extra(),

            TdType::Update(value) => value.extra(),

            TdType::AccountTtl(value) => value.extra(),

            TdType::Animations(value) => value.extra(),

            TdType::AuthenticationCodeInfo(value) => value.extra(),

            TdType::AutoDownloadSettingsPresets(value) => value.extra(),

            TdType::Background(value) => value.extra(),

            TdType::Backgrounds(value) => value.extra(),

            TdType::BankCardInfo(value) => value.extra(),

            TdType::BasicGroup(value) => value.extra(),

            TdType::BasicGroupFullInfo(value) => value.extra(),

            TdType::CallId(value) => value.extra(),

            TdType::CallbackQueryAnswer(value) => value.extra(),

            TdType::Chat(value) => value.extra(),

            TdType::ChatAdministrators(value) => value.extra(),

            TdType::ChatEvents(value) => value.extra(),

            TdType::ChatFilter(value) => value.extra(),

            TdType::ChatFilterInfo(value) => value.extra(),

            TdType::ChatInviteLink(value) => value.extra(),

            TdType::ChatInviteLinkInfo(value) => value.extra(),

            TdType::ChatLists(value) => value.extra(),

            TdType::ChatMember(value) => value.extra(),

            TdType::ChatMembers(value) => value.extra(),

            TdType::ChatPhotos(value) => value.extra(),

            TdType::Chats(value) => value.extra(),

            TdType::ChatsNearby(value) => value.extra(),

            TdType::ConnectedWebsites(value) => value.extra(),

            TdType::Count(value) => value.extra(),

            TdType::Countries(value) => value.extra(),

            TdType::CustomRequestResult(value) => value.extra(),

            TdType::DatabaseStatistics(value) => value.extra(),

            TdType::DeepLinkInfo(value) => value.extra(),

            TdType::EmailAddressAuthenticationCodeInfo(value) => value.extra(),

            TdType::Emojis(value) => value.extra(),

            TdType::Error(value) => value.extra(),

            TdType::File(value) => value.extra(),

            TdType::FilePart(value) => value.extra(),

            TdType::FormattedText(value) => value.extra(),

            TdType::FoundMessages(value) => value.extra(),

            TdType::GameHighScores(value) => value.extra(),

            TdType::Hashtags(value) => value.extra(),

            TdType::HttpUrl(value) => value.extra(),

            TdType::ImportedContacts(value) => value.extra(),

            TdType::InlineQueryResults(value) => value.extra(),

            TdType::LanguagePackInfo(value) => value.extra(),

            TdType::LanguagePackStrings(value) => value.extra(),

            TdType::LocalizationTargetInfo(value) => value.extra(),

            TdType::LogTags(value) => value.extra(),

            TdType::LogVerbosityLevel(value) => value.extra(),

            TdType::Message(value) => value.extra(),

            TdType::MessageLink(value) => value.extra(),

            TdType::MessageLinkInfo(value) => value.extra(),

            TdType::MessageSenders(value) => value.extra(),

            TdType::MessageStatistics(value) => value.extra(),

            TdType::MessageThreadInfo(value) => value.extra(),

            TdType::Messages(value) => value.extra(),

            TdType::NetworkStatistics(value) => value.extra(),

            TdType::Ok(value) => value.extra(),

            TdType::OrderInfo(value) => value.extra(),

            TdType::PassportAuthorizationForm(value) => value.extra(),

            TdType::PassportElements(value) => value.extra(),

            TdType::PassportElementsWithErrors(value) => value.extra(),

            TdType::PasswordState(value) => value.extra(),

            TdType::PaymentForm(value) => value.extra(),

            TdType::PaymentReceipt(value) => value.extra(),

            TdType::PaymentResult(value) => value.extra(),

            TdType::PhoneNumberInfo(value) => value.extra(),

            TdType::Proxies(value) => value.extra(),

            TdType::Proxy(value) => value.extra(),

            TdType::PushReceiverId(value) => value.extra(),

            TdType::RecommendedChatFilters(value) => value.extra(),

            TdType::RecoveryEmailAddress(value) => value.extra(),

            TdType::ScopeNotificationSettings(value) => value.extra(),

            TdType::Seconds(value) => value.extra(),

            TdType::SecretChat(value) => value.extra(),

            TdType::Session(value) => value.extra(),

            TdType::Sessions(value) => value.extra(),

            TdType::StickerSet(value) => value.extra(),

            TdType::StickerSets(value) => value.extra(),

            TdType::Stickers(value) => value.extra(),

            TdType::StorageStatistics(value) => value.extra(),

            TdType::StorageStatisticsFast(value) => value.extra(),

            TdType::Supergroup(value) => value.extra(),

            TdType::SupergroupFullInfo(value) => value.extra(),

            TdType::TMeUrls(value) => value.extra(),

            TdType::TemporaryPasswordState(value) => value.extra(),

            TdType::TestBytes(value) => value.extra(),

            TdType::TestInt(value) => value.extra(),

            TdType::TestString(value) => value.extra(),

            TdType::TestVectorInt(value) => value.extra(),

            TdType::TestVectorIntObject(value) => value.extra(),

            TdType::TestVectorString(value) => value.extra(),

            TdType::TestVectorStringObject(value) => value.extra(),

            TdType::Text(value) => value.extra(),

            TdType::TextEntities(value) => value.extra(),

            TdType::Updates(value) => value.extra(),

            TdType::User(value) => value.extra(),

            TdType::UserFullInfo(value) => value.extra(),

            TdType::UserPrivacySettingRules(value) => value.extra(),

            TdType::Users(value) => value.extra(),

            TdType::ValidatedOrderInfo(value) => value.extra(),

            TdType::WebPage(value) => value.extra(),

            TdType::WebPageInstantView(value) => value.extra(),
        }
    }
}

pub(super) fn number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn vec_of_i64_from_str<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Vec::<String>::deserialize(deserializer)?;
    let mut r = Vec::new();
    for v in s {
        match v.parse::<i64>() {
            Ok(v) => r.push(v),
            Err(e) => return Err(D::Error::custom(format!("can't deserialize to i64: {}", e))),
        }
    }
    Ok(r)
}

#[cfg(test)]
mod tests {
    use crate::types::_common::deserialize_update;
    use crate::types::{from_json, AuthorizationState, TdType, Update, UpdateAuthorizationState};

    #[test]
    fn test_deserialize_enums() {
        match deserialize_update(
            "updateAuthorizationState", serde_json::from_str::<serde_json::Value>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#).unwrap(),
        ) {
            Ok(v) => {match v {
                Some(v) => {
                    match v {
                        TdType::Update(_) => {},

                        _ => {panic!("serialization failed")},
                    }
                },
                None => panic!("serialization failed")
            }}
            Err(e) => {
                panic!("{}", e)
            }
        };

        match from_json::<TdType>(
            r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#,
        ) {
            Ok(t) => match t {
                TdType::Update(Update::AuthorizationState(state)) => {
                    match state.authorization_state() {
                        AuthorizationState::WaitTdlibParameters(_) => {}
                        _ => {
                            panic!("invalid serialized data")
                        }
                    }
                }
                _ => panic!("from_json failed: {:?}", t),
            },
            Err(e) => {
                panic!("{}", e)
            }
        };
    }
}
